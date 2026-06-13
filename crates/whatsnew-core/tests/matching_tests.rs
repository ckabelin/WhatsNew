use whatsnew_core::db::{feeds, topics, Db};
use whatsnew_core::matching::ensure_feeds_for_topic;

#[tokio::test]
async fn unmatched_topic_still_gets_a_fallback_feed() {
    let db = Db::connect_in_memory().await.unwrap();
    let topic = topics::create(&db.pool, "Formula 1").await.unwrap();

    ensure_feeds_for_topic(&db.pool, &topic).await.unwrap();

    let topic_feeds = feeds::list_for_topic(&db.pool, topic.id).await.unwrap();
    assert!(!topic_feeds.is_empty());
    assert!(topic_feeds
        .iter()
        .any(|f| f.url.starts_with("https://news.google.com/rss/search")));
}

#[tokio::test]
async fn matched_topic_gets_curated_feeds_plus_fallback() {
    let db = Db::connect_in_memory().await.unwrap();
    let topic = topics::create(&db.pool, "Rust programming").await.unwrap();

    ensure_feeds_for_topic(&db.pool, &topic).await.unwrap();

    let topic_feeds = feeds::list_for_topic(&db.pool, topic.id).await.unwrap();
    assert!(topic_feeds
        .iter()
        .any(|f| f.url.starts_with("https://news.google.com/rss/search")));
    assert!(topic_feeds
        .iter()
        .any(|f| f.url.contains("hnrss.org") || f.url.contains("lobste.rs")));
}

#[tokio::test]
async fn ensure_feeds_is_idempotent() {
    let db = Db::connect_in_memory().await.unwrap();
    let topic = topics::create(&db.pool, "Formula 1").await.unwrap();

    ensure_feeds_for_topic(&db.pool, &topic).await.unwrap();
    let first = feeds::list_for_topic(&db.pool, topic.id).await.unwrap();

    ensure_feeds_for_topic(&db.pool, &topic).await.unwrap();
    let second = feeds::list_for_topic(&db.pool, topic.id).await.unwrap();

    assert_eq!(first.len(), second.len());
}
