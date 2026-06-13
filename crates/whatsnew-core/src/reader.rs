use reqwest::Client;
use scraper::{ElementRef, Html, Selector};
use serde::{Deserialize, Serialize};

use crate::db::articles;
use crate::error::{CoreError, Result};
use crate::models::Article;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadableImage {
    pub url: String,
    pub alt: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ReadableBlock {
    Paragraph { text: String },
    Image { image: ReadableImage },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadableArticle {
    pub article: Article,
    pub source_url: String,
    pub title: String,
    pub paragraphs: Vec<String>,
    pub images: Vec<ReadableImage>,
    pub content: Vec<ReadableBlock>,
}

pub async fn read_article(
    pool: &sqlx::SqlitePool,
    client: &Client,
    article_id: i64,
) -> Result<ReadableArticle> {
    let article = articles::get(pool, article_id).await?;
    let source_url = article
        .link
        .clone()
        .ok_or_else(|| CoreError::InvalidInput("article has no source URL".to_string()))?;
    let url = reqwest::Url::parse(&source_url)
        .map_err(|e| CoreError::InvalidInput(format!("invalid article URL: {e}")))?;

    match url.scheme() {
        "http" | "https" => {}
        scheme => {
            return Err(CoreError::InvalidInput(format!(
                "unsupported article URL scheme '{scheme}'"
            )));
        }
    }

    let html = client
        .get(url.clone())
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;
    let extracted = extract_readable_content(&html, &url, &article.title);

    Ok(ReadableArticle {
        article,
        source_url,
        title: extracted.title,
        paragraphs: extracted.paragraphs,
        images: extracted.images,
        content: extracted.content,
    })
}

struct ExtractedContent {
    title: String,
    paragraphs: Vec<String>,
    images: Vec<ReadableImage>,
    content: Vec<ReadableBlock>,
}

fn extract_readable_content(
    html: &str,
    base_url: &reqwest::Url,
    fallback_title: &str,
) -> ExtractedContent {
    let document = Html::parse_document(html);
    let title = extract_title(&document).unwrap_or_else(|| fallback_title.to_string());
    let mut content = extract_ordered_content(&document, base_url);
    let mut paragraphs = paragraphs_from_blocks(&content);
    let mut images = images_from_blocks(&content);

    if paragraphs.is_empty() {
        paragraphs = extract_paragraphs(&document);
    }

    if images.is_empty() {
        images = extract_images(&document, base_url);
    }

    if content.is_empty() {
        content = paragraphs
            .iter()
            .cloned()
            .map(|text| ReadableBlock::Paragraph { text })
            .collect();
    }

    ExtractedContent {
        title,
        paragraphs,
        images,
        content,
    }
}

fn extract_title(document: &Html) -> Option<String> {
    for selector in [
        r#"meta[property="og:title"]"#,
        r#"meta[name="twitter:title"]"#,
        "h1",
        "title",
    ] {
        let selector = Selector::parse(selector).expect("static selector is valid");
        for element in document.select(&selector) {
            let value = element
                .value()
                .attr("content")
                .map(str::to_string)
                .unwrap_or_else(|| normalized_text(element.text()));
            if !value.is_empty() {
                return Some(value);
            }
        }
    }

    None
}

fn extract_paragraphs(document: &Html) -> Vec<String> {
    let container_selector = Selector::parse(
        r#"article, main, [role="main"], .article, .article-content, .entry-content, .post, .story-body"#,
    )
    .expect("static selector is valid");
    let body_selector = Selector::parse("body").expect("static selector is valid");
    let paragraph_selector = Selector::parse("p").expect("static selector is valid");

    let best = best_paragraph_candidate(document.select(&container_selector), &paragraph_selector);
    if !best.is_empty() {
        return best;
    }

    best_paragraph_candidate(document.select(&body_selector), &paragraph_selector)
}

fn extract_ordered_content(document: &Html, base_url: &reqwest::Url) -> Vec<ReadableBlock> {
    let Some(container) = best_content_container(document) else {
        return Vec::new();
    };

    let mut blocks = Vec::new();
    let mut seen_images = Vec::new();
    collect_blocks_in_order(container, base_url, &mut blocks, &mut seen_images);

    blocks
}

fn best_content_container(document: &Html) -> Option<ElementRef<'_>> {
    let container_selector = Selector::parse(
        r#"article, main, [role="main"], .article, .article-content, .entry-content, .post, .story-body"#,
    )
    .expect("static selector is valid");
    let body_selector = Selector::parse("body").expect("static selector is valid");
    let paragraph_selector = Selector::parse("p").expect("static selector is valid");

    let best = document
        .select(&container_selector)
        .max_by_key(|container| {
            collect_paragraphs(*container, &paragraph_selector)
                .iter()
                .map(String::len)
                .sum::<usize>()
        });

    best.or_else(|| document.select(&body_selector).next())
}

fn collect_blocks_in_order(
    element: ElementRef<'_>,
    base_url: &reqwest::Url,
    blocks: &mut Vec<ReadableBlock>,
    seen_images: &mut Vec<String>,
) {
    if element.value().name() == "p" {
        let text = normalized_text(element.text());
        if is_readable_paragraph(&text) {
            blocks.push(ReadableBlock::Paragraph { text });
        }
        return;
    }

    if element.value().name() == "img" {
        if let Some(image) = readable_image_from_element(element, base_url) {
            if !seen_images.iter().any(|url| url == &image.url) && seen_images.len() < 6 {
                seen_images.push(image.url.clone());
                blocks.push(ReadableBlock::Image { image });
            }
        }
        return;
    }

    for child in element.children() {
        let Some(child_element) = ElementRef::wrap(child) else {
            continue;
        };

        if should_skip_element(child_element.value().name()) {
            continue;
        }

        collect_blocks_in_order(child_element, base_url, blocks, seen_images);
    }
}

fn should_skip_element(name: &str) -> bool {
    matches!(
        name,
        "aside"
            | "button"
            | "footer"
            | "form"
            | "header"
            | "iframe"
            | "nav"
            | "noscript"
            | "script"
            | "style"
            | "svg"
            | "video"
    )
}

fn paragraphs_from_blocks(blocks: &[ReadableBlock]) -> Vec<String> {
    blocks
        .iter()
        .filter_map(|block| match block {
            ReadableBlock::Paragraph { text } => Some(text.clone()),
            ReadableBlock::Image { .. } => None,
        })
        .collect()
}

fn images_from_blocks(blocks: &[ReadableBlock]) -> Vec<ReadableImage> {
    blocks
        .iter()
        .filter_map(|block| match block {
            ReadableBlock::Paragraph { .. } => None,
            ReadableBlock::Image { image } => Some(image.clone()),
        })
        .collect()
}

fn extract_images(document: &Html, base_url: &reqwest::Url) -> Vec<ReadableImage> {
    let container_selector = Selector::parse(
        r#"article, main, [role="main"], .article, .article-content, .entry-content, .post, .story-body"#,
    )
    .expect("static selector is valid");
    let body_selector = Selector::parse("body").expect("static selector is valid");
    let image_selector = Selector::parse("img").expect("static selector is valid");

    let mut images = collect_images_from_containers(
        document.select(&container_selector),
        &image_selector,
        base_url,
    );

    if images.is_empty() {
        images = collect_images_from_containers(
            document.select(&body_selector),
            &image_selector,
            base_url,
        );
    }

    if images.is_empty() {
        if let Some(image) = extract_meta_image(document, base_url) {
            images.push(image);
        }
    }

    images.truncate(6);
    images
}

fn collect_images_from_containers<'a>(
    containers: impl Iterator<Item = ElementRef<'a>>,
    image_selector: &Selector,
    base_url: &reqwest::Url,
) -> Vec<ReadableImage> {
    let mut images: Vec<ReadableImage> = Vec::new();

    for container in containers {
        for image in container.select(image_selector) {
            if let Some(readable_image) = readable_image_from_element(image, base_url) {
                if !images
                    .iter()
                    .any(|existing| existing.url == readable_image.url)
                {
                    images.push(readable_image);
                }
            }
        }
    }

    images
}

fn extract_meta_image(document: &Html, base_url: &reqwest::Url) -> Option<ReadableImage> {
    for selector in [
        r#"meta[property="og:image"]"#,
        r#"meta[name="twitter:image"]"#,
        r#"meta[property="twitter:image"]"#,
    ] {
        let selector = Selector::parse(selector).expect("static selector is valid");
        for element in document.select(&selector) {
            let Some(raw_url) = element.value().attr("content") else {
                continue;
            };
            let Ok(url) = base_url.join(raw_url) else {
                continue;
            };
            if !looks_like_non_article_image(url.as_ref(), None) {
                return Some(ReadableImage {
                    url: url.to_string(),
                    alt: None,
                });
            }
        }
    }

    None
}

fn readable_image_from_element(
    image: ElementRef<'_>,
    base_url: &reqwest::Url,
) -> Option<ReadableImage> {
    let value = image.value();
    let raw_url = value
        .attr("src")
        .or_else(|| value.attr("data-src"))
        .or_else(|| value.attr("data-original"))
        .or_else(|| value.attr("data-lazy-src"))
        .or_else(|| value.attr("srcset").and_then(first_srcset_url))?;

    let url = base_url.join(raw_url).ok()?;
    let url = url.to_string();
    let alt = value
        .attr("alt")
        .map(str::trim)
        .filter(|alt| !alt.is_empty())
        .map(String::from);

    if is_tiny_image(value.attr("width"), value.attr("height"))
        || looks_like_non_article_image(&url, alt.as_deref())
        || element_metadata_suggests_ad(value.attr("class"), value.attr("id"))
    {
        return None;
    }

    Some(ReadableImage { url, alt })
}

fn first_srcset_url(srcset: &str) -> Option<&str> {
    srcset
        .split(',')
        .filter_map(|candidate| candidate.split_whitespace().next())
        .find(|candidate| !candidate.is_empty())
}

fn is_tiny_image(width: Option<&str>, height: Option<&str>) -> bool {
    let width = width.and_then(|value| value.parse::<u32>().ok());
    let height = height.and_then(|value| value.parse::<u32>().ok());

    matches!((width, height), (Some(w), Some(h)) if w < 120 || h < 80)
}

fn looks_like_non_article_image(url: &str, alt: Option<&str>) -> bool {
    let haystack = format!("{} {}", url, alt.unwrap_or("")).to_lowercase();
    [
        "advert",
        "/ad/",
        "-ad-",
        "_ad_",
        "/ads",
        "ads.",
        "sponsor",
        "promo",
        "banner",
        "pixel",
        "tracker",
        "tracking",
        "analytics",
        "logo",
        "avatar",
        "icon",
        "sprite",
        "social",
        "share",
    ]
    .iter()
    .any(|term| haystack.contains(term))
}

fn element_metadata_suggests_ad(class: Option<&str>, id: Option<&str>) -> bool {
    let haystack = format!("{} {}", class.unwrap_or(""), id.unwrap_or("")).to_lowercase();
    [
        "advert", "ad-slot", "ad_", "ads", "sponsor", "promo", "banner", "tracking", "social",
        "share",
    ]
    .iter()
    .any(|term| haystack.contains(term))
}

fn best_paragraph_candidate<'a>(
    containers: impl Iterator<Item = ElementRef<'a>>,
    paragraph_selector: &Selector,
) -> Vec<String> {
    let mut best = Vec::new();
    let mut best_score = 0usize;

    for container in containers {
        let paragraphs = collect_paragraphs(container, paragraph_selector);
        let score = paragraphs.iter().map(String::len).sum();
        if score > best_score {
            best = paragraphs;
            best_score = score;
        }
    }

    best
}

fn collect_paragraphs(container: ElementRef<'_>, paragraph_selector: &Selector) -> Vec<String> {
    let mut paragraphs = Vec::new();

    for paragraph in container.select(paragraph_selector) {
        let text = normalized_text(paragraph.text());
        if is_readable_paragraph(&text) {
            paragraphs.push(text);
        }
    }

    paragraphs
}

fn normalized_text<'a>(parts: impl Iterator<Item = &'a str>) -> String {
    parts
        .flat_map(str::split_whitespace)
        .collect::<Vec<_>>()
        .join(" ")
        .trim()
        .to_string()
}

fn is_readable_paragraph(text: &str) -> bool {
    let lower = text.to_lowercase();
    text.chars().count() >= 40
        && !lower.contains("enable javascript")
        && !lower.contains("sign up for")
        && !lower.contains("subscribe to")
        && !lower.contains("cookie")
}

#[cfg(test)]
mod tests {
    use super::{extract_readable_content, ReadableBlock};

    #[test]
    fn extracts_title_and_article_paragraphs() {
        let base_url = reqwest::Url::parse("https://example.com/news/story").unwrap();
        let html = r#"
            <html>
              <head><meta property="og:title" content="Readable title"></head>
              <body>
                <nav><p>This navigation paragraph is intentionally long but should lose.</p></nav>
                <article>
                  <p>This is the first real paragraph with enough words to pass the filter.</p>
                  <p>This is the second real paragraph, also long enough to be included.</p>
                </article>
              </body>
            </html>
        "#;

        let content = extract_readable_content(html, &base_url, "Fallback");

        assert_eq!(content.title, "Readable title");
        assert_eq!(content.paragraphs.len(), 2);
        assert_eq!(content.content.len(), 2);
        assert!(content.paragraphs[0].starts_with("This is the first real"));
    }

    #[test]
    fn extracts_article_images_and_filters_ad_images() {
        let base_url = reqwest::Url::parse("https://example.com/news/story").unwrap();
        let html = r#"
            <html>
              <head><meta property="og:image" content="/fallback.jpg"></head>
              <body>
                <article>
                  <p>This opening paragraph gives context before the main image appears.</p>
                  <img src="/images/story-main.jpg" alt="Main story image" width="800" height="500">
                  <p>This follow-up paragraph explains what the image means for readers.</p>
                  <img src="/ads/banner.jpg" alt="Advertisement" width="728" height="90">
                  <img src="/images/tracker.gif" width="1" height="1">
                </article>
              </body>
            </html>
        "#;

        let content = extract_readable_content(html, &base_url, "Fallback");

        assert_eq!(content.images.len(), 1);
        assert_eq!(content.content.len(), 3);
        assert_eq!(
            content.images[0].url,
            "https://example.com/images/story-main.jpg"
        );
        assert_eq!(content.images[0].alt.as_deref(), Some("Main story image"));
        assert!(matches!(
            content.content[0],
            ReadableBlock::Paragraph { .. }
        ));
        assert!(matches!(content.content[1], ReadableBlock::Image { .. }));
        assert!(matches!(
            content.content[2],
            ReadableBlock::Paragraph { .. }
        ));
    }
}
