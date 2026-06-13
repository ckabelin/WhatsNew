use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

use crate::error::{CoreError, Result};

/// A feed discovered by scanning a page's `<link rel="alternate">` tags.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredFeed {
    pub url: String,
    pub title: Option<String>,
}

/// Scans `html` (the page at `base_url`) for `<link rel="alternate" type="application/rss+xml|atom+xml">`
/// tags and returns their resolved absolute URLs.
pub fn discover_feeds(base_url: &str, html: &str) -> Result<Vec<DiscoveredFeed>> {
    let base = reqwest::Url::parse(base_url)
        .map_err(|e| CoreError::InvalidInput(format!("invalid base url '{base_url}': {e}")))?;

    let document = Html::parse_document(html);
    let selector = Selector::parse(r#"link[rel="alternate"]"#).expect("static selector is valid");

    let mut feeds = Vec::new();
    for el in document.select(&selector) {
        let value = el.value();
        let feed_type = value.attr("type").unwrap_or("");
        if feed_type != "application/rss+xml" && feed_type != "application/atom+xml" {
            continue;
        }

        let Some(href) = value.attr("href") else {
            continue;
        };
        let Ok(url) = base.join(href) else { continue };

        feeds.push(DiscoveredFeed {
            url: url.to_string(),
            title: value.attr("title").map(String::from),
        });
    }

    Ok(feeds)
}
