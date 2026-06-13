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

const GOOGLE_NEWS_RSS: &str = r##"<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0">
  <channel>
    <title>Google News</title>
    <link>https://news.google.com</link>
    <item>
      <title>Breaking Story Headline</title>
      <link>https://news.google.com/articles/abc</link>
      <guid>https://news.google.com/articles/abc</guid>
      <pubDate>Mon, 01 Jan 2024 00:00:00 GMT</pubDate>
      <description>&lt;a href="https://example.com/abc" target="_blank"&gt;Breaking Story Headline&lt;/a&gt;&amp;nbsp;&amp;nbsp;&lt;font color="#6f6f6f"&gt;Example News&lt;/font&gt;</description>
    </item>
  </channel>
</rss>"##;

#[test]
fn drops_title_only_google_news_summary() {
    let feed = parse::parse_feed(GOOGLE_NEWS_RSS.as_bytes()).unwrap();
    let articles = parse::to_new_articles(&feed);
    assert_eq!(articles.len(), 1);
    assert_eq!(articles[0].title, "Breaking Story Headline");
    assert_eq!(articles[0].summary, None);
    // The Google News redirect link is replaced with the original article URL
    // embedded in the description's <a href>.
    assert_eq!(
        articles[0].link,
        Some("https://example.com/abc".to_string())
    );
}

const DOUBLE_ENCODED_GOOGLE_NEWS_RSS: &str = r##"<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0">
  <channel>
    <title>Google News</title>
    <link>https://news.google.com</link>
    <item>
      <title>Different Headline</title>
      <link>https://news.google.com/articles/xyz</link>
      <guid>https://news.google.com/articles/xyz</guid>
      <pubDate>Mon, 01 Jan 2024 00:00:00 GMT</pubDate>
      <description>&amp;lt;a href=&amp;quot;https://example.com/xyz&amp;quot;&amp;gt;Some Article Title With More Words Here&amp;lt;/a&amp;gt;</description>
    </item>
  </channel>
</rss>"##;

#[test]
fn strips_double_encoded_html_and_recovers_original_link() {
    let feed = parse::parse_feed(DOUBLE_ENCODED_GOOGLE_NEWS_RSS.as_bytes()).unwrap();
    let articles = parse::to_new_articles(&feed);
    assert_eq!(articles.len(), 1);
    assert_eq!(
        articles[0].summary,
        Some("Some Article Title With More Words Here".to_string())
    );
    assert_eq!(
        articles[0].link,
        Some("https://example.com/xyz".to_string())
    );
}

const DUPLICATE_SENTENCE_RSS: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0">
  <channel>
    <title>Example Feed</title>
    <link>https://example.com</link>
    <item>
      <title>Second Post</title>
      <link>https://example.com/second</link>
      <guid>https://example.com/second</guid>
      <pubDate>Mon, 01 Jan 2024 00:00:00 GMT</pubDate>
      <description>Prices rose sharply today. Prices rose sharply today. Analysts expect the trend to continue.</description>
    </item>
  </channel>
</rss>"#;

#[test]
fn dedupes_repeated_leading_sentence_in_summary() {
    let feed = parse::parse_feed(DUPLICATE_SENTENCE_RSS.as_bytes()).unwrap();
    let articles = parse::to_new_articles(&feed);
    assert_eq!(articles.len(), 1);
    assert_eq!(
        articles[0].summary,
        Some("Prices rose sharply today. Analysts expect the trend to continue.".to_string())
    );
}
