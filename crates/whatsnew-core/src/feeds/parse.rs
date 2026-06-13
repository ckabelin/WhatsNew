use feed_rs::model::Feed as RawFeed;
use scraper::{Html, Selector};

use crate::db::articles::NewArticle;
use crate::error::Result;

/// Parses raw RSS/Atom/JSON-Feed bytes into `feed-rs`'s unified feed model.
pub fn parse_feed(bytes: &[u8]) -> Result<RawFeed> {
    Ok(feed_rs::parser::parse(bytes)?)
}

/// Returns the feed's display title, if present.
pub fn feed_title(feed: &RawFeed) -> Option<String> {
    feed.title.as_ref().map(|t| t.content.clone())
}

/// Converts a parsed feed's entries into rows ready for `articles::insert_new`.
pub fn to_new_articles(feed: &RawFeed) -> Vec<NewArticle> {
    feed.entries
        .iter()
        .map(|entry| {
            let title = entry
                .title
                .as_ref()
                .map(|t| t.content.clone())
                .unwrap_or_else(|| "(untitled)".to_string());

            let link = entry.links.first().map(|l| l.href.clone());

            let raw_summary = entry
                .summary
                .as_ref()
                .map(|s| s.content.clone())
                .or_else(|| entry.content.as_ref().and_then(|c| c.body.clone()));

            let link = resolve_link(link, raw_summary.as_deref());
            let summary = clean_summary(&title, raw_summary);

            let published_at = entry.published.or(entry.updated);

            NewArticle {
                guid: entry.id.clone(),
                title,
                link,
                summary,
                published_at,
            }
        })
        .collect()
}

/// Cleans a feed entry's raw summary/content into plain text suitable for
/// display.
///
/// Strips HTML markup, collapses whitespace, drops a duplicated leading
/// sentence (seen in some feeds), and discards summaries that turn out to be
/// just the article title plus a short source attribution - a pattern common
/// in Google News RSS, whose `<description>` is only a link to the title
/// followed by the publication name.
pub fn clean_summary(title: &str, raw: Option<String>) -> Option<String> {
    let text = raw?;
    let stripped = strip_html(&text);
    let deduped = dedupe_leading_sentence(&stripped);
    let trimmed = deduped.trim();
    if trimmed.is_empty() {
        return None;
    }

    let title_trim = title.trim();
    if trimmed.eq_ignore_ascii_case(title_trim) {
        return None;
    }
    if let Some(rest) = trimmed.strip_prefix(title_trim) {
        let rest = rest.trim_start_matches([' ', '-', '|', '\u{b7}']);
        if rest.len() < 30 {
            return None;
        }
    }

    Some(trimmed.to_string())
}

/// If `link` points at a Google News redirect (`news.google.com`), tries to
/// replace it with the original article URL embedded as an `<a href>` inside
/// the entry's raw summary/description - Google News redirect links are
/// obfuscated and don't resolve to the source article on their own, so
/// surfacing the embedded link lets the user open/read the original.
pub fn resolve_link(link: Option<String>, raw_summary: Option<&str>) -> Option<String> {
    match &link {
        Some(href) if href.contains("news.google.com") => {
            raw_summary.and_then(extract_first_link).or(link)
        }
        _ => link,
    }
}

/// Extracts the `href` of the first link in `input`, decoding double-escaped
/// HTML markup (seen in some feed descriptions, where `<a href="...">` ends
/// up entity-encoded twice and would otherwise render as literal tag text)
/// the same way `strip_html` does.
pub fn extract_first_link(input: &str) -> Option<String> {
    let selector = Selector::parse("a[href]").ok()?;
    let mut current = input.to_string();
    for _ in 0..3 {
        let fragment = Html::parse_fragment(&current);
        if let Some(el) = fragment.select(&selector).next() {
            return el.value().attr("href").map(|s| s.to_string());
        }

        let text = collapse_whitespace(&fragment);
        if text == current || !looks_like_html(&text) {
            return None;
        }
        current = text;
    }
    None
}

/// Strips HTML tags, decoding entities and collapsing whitespace in the
/// process. Some feeds (notably Google News) double-encode markup in
/// descriptions, e.g. `&amp;lt;a href=...&amp;gt;`, which after a single pass
/// of HTML parsing leaves literal `<a href=...>` text behind rather than a
/// real element. Re-parsing such leftover text up to a couple more times
/// strips that layer too.
fn strip_html(input: &str) -> String {
    let mut current = input.to_string();
    for _ in 0..3 {
        let fragment = Html::parse_fragment(&current);
        let text = collapse_whitespace(&fragment);
        if text == current || !looks_like_html(&text) {
            return text;
        }
        current = text;
    }
    current
}

fn collapse_whitespace(fragment: &Html) -> String {
    let text: String = fragment.root_element().text().collect::<Vec<_>>().join(" ");
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// True if `text` contains what looks like a leftover HTML tag (`<` followed
/// by a letter or `/`), as opposed to a stray `<`/`>` used as e.g. a
/// comparison operator.
fn looks_like_html(text: &str) -> bool {
    text.match_indices('<').any(|(i, _)| {
        text[i + 1..]
            .chars()
            .next()
            .is_some_and(|c| c.is_ascii_alphabetic() || c == '/')
    })
}

/// If `text` starts with the same sentence twice in a row (e.g. "Foo bar.
/// Foo bar. Baz."), drops the duplicate leading copy.
fn dedupe_leading_sentence(text: &str) -> String {
    let text = text.trim();
    if let Some(idx) = text.find(['.', '!', '?']) {
        let (first, rest) = text.split_at(idx + 1);
        let first_trimmed = first.trim();
        let rest_trimmed = rest.trim_start();
        if !first_trimmed.is_empty() && rest_trimmed.starts_with(first_trimmed) {
            return rest_trimmed.to_string();
        }
    }
    text.to_string()
}
