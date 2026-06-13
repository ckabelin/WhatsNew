use whatsnew_core::feeds::{discovery, parse};

const SAMPLE_RSS: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0">
  <channel>
    <title>Example Feed</title>
    <link>https://example.com</link>
    <item>
      <title>First Post</title>
      <link>https://example.com/first</link>
      <guid>https://example.com/first</guid>
      <pubDate>Mon, 01 Jan 2024 00:00:00 GMT</pubDate>
      <description>First post summary</description>
    </item>
  </channel>
</rss>"#;

#[test]
fn parses_rss_feed_into_articles() {
    let feed = parse::parse_feed(SAMPLE_RSS.as_bytes()).unwrap();
    assert_eq!(parse::feed_title(&feed), Some("Example Feed".to_string()));

    let articles = parse::to_new_articles(&feed);
    assert_eq!(articles.len(), 1);
    assert_eq!(articles[0].title, "First Post");
    assert_eq!(
        articles[0].link,
        Some("https://example.com/first".to_string())
    );
    assert!(articles[0].published_at.is_some());
}

const SAMPLE_HTML: &str = r#"<!DOCTYPE html>
<html>
<head>
  <link rel="alternate" type="application/rss+xml" title="Example RSS" href="/feed.xml">
  <link rel="stylesheet" href="/style.css">
</head>
<body></body>
</html>"#;

#[test]
fn discovers_feed_links_in_html() {
    let feeds = discovery::discover_feeds("https://example.com/blog", SAMPLE_HTML).unwrap();
    assert_eq!(feeds.len(), 1);
    assert_eq!(feeds[0].url, "https://example.com/feed.xml");
    assert_eq!(feeds[0].title, Some("Example RSS".to_string()));
}
