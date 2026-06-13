use whatsnew_core::db::articles::NewArticle;
use whatsnew_core::db::{articles, feeds, settings, topics, Db};

#[tokio::test]
async fn topic_crud_and_settings_roundtrip() {
    let db = Db::connect_in_memory().await.unwrap();

    let topic = topics::create(&db.pool, "Rust Programming").await.unwrap();
    assert_eq!(topic.name, "Rust Programming");
    assert!(topic.notifications_enabled);
    assert!(!topic.initial_refresh_done);

    let all = topics::list(&db.pool).await.unwrap();
    assert_eq!(all.len(), 1);

    let feed = feeds::get_or_create(&db.pool, "https://hnrss.org/frontpage", Some("HN"), None)
        .await
        .unwrap();
    feeds::link_to_topic(&db.pool, topic.id, feed.id)
        .await
        .unwrap();

    let topic_feeds = feeds::list_for_topic(&db.pool, topic.id).await.unwrap();
    assert_eq!(topic_feeds.len(), 1);

    let new_articles = vec![NewArticle {
        guid: "guid-1".into(),
        title: "Hello world".into(),
        link: Some("https://example.com/1".into()),
        summary: None,
        published_at: None,
    }];
    let inserted = articles::insert_new(&db.pool, feed.id, &new_articles)
        .await
        .unwrap();
    assert_eq!(inserted.len(), 1);

    // Inserting the same guid again is a no-op.
    let inserted_again = articles::insert_new(&db.pool, feed.id, &new_articles)
        .await
        .unwrap();
    assert!(inserted_again.is_empty());

    let topic_articles = articles::list_for_topic(&db.pool, topic.id, 10)
        .await
        .unwrap();
    assert_eq!(topic_articles.len(), 1);

    let current_settings = settings::get(&db.pool).await.unwrap();
    assert_eq!(current_settings.retention_days, 30);

    let mut updated = current_settings.clone();
    updated.retention_days = 7;
    let saved = settings::update(&db.pool, &updated).await.unwrap();
    assert_eq!(saved.retention_days, 7);

    topics::delete(&db.pool, topic.id).await.unwrap();
    assert!(topics::list(&db.pool).await.unwrap().is_empty());
}

#[tokio::test]
async fn topic_order_can_be_rearranged() {
    let db = Db::connect_in_memory().await.unwrap();

    let rust = topics::create(&db.pool, "Rust").await.unwrap();
    let design = topics::create(&db.pool, "Design").await.unwrap();
    let security = topics::create(&db.pool, "Security").await.unwrap();

    let initial = topics::list(&db.pool).await.unwrap();
    assert_eq!(
        initial.iter().map(|topic| topic.id).collect::<Vec<_>>(),
        vec![rust.id, design.id, security.id]
    );

    let reordered = topics::reorder(&db.pool, &[security.id, rust.id, design.id])
        .await
        .unwrap();
    assert_eq!(
        reordered.iter().map(|topic| topic.id).collect::<Vec<_>>(),
        vec![security.id, rust.id, design.id]
    );

    let listed = topics::list(&db.pool).await.unwrap();
    assert_eq!(
        listed.iter().map(|topic| topic.id).collect::<Vec<_>>(),
        vec![security.id, rust.id, design.id]
    );

    let invalid = topics::reorder(&db.pool, &[security.id, rust.id]).await;
    assert!(invalid.is_err());
}
