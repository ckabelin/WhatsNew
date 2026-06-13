use chrono::{Duration, Utc};
use whatsnew_core::db::articles::NewArticle;
use whatsnew_core::db::{articles, feeds, settings, topics, Db};
use whatsnew_core::retention;

#[tokio::test]
async fn prunes_old_and_excess_articles() {
    let db = Db::connect_in_memory().await.unwrap();
    let topic = topics::create(&db.pool, "Test").await.unwrap();
    let feed = feeds::get_or_create(&db.pool, "https://example.com/feed.xml", None, None)
        .await
        .unwrap();
    feeds::link_to_topic(&db.pool, topic.id, feed.id)
        .await
        .unwrap();

    let old_article = NewArticle {
        guid: "old".into(),
        title: "Old".into(),
        link: None,
        summary: None,
        published_at: Some(Utc::now() - Duration::days(40)),
    };
    let recent_article = NewArticle {
        guid: "recent".into(),
        title: "Recent".into(),
        link: None,
        summary: None,
        published_at: Some(Utc::now()),
    };
    articles::insert_new(&db.pool, feed.id, &[old_article, recent_article])
        .await
        .unwrap();

    let mut current_settings = settings::get(&db.pool).await.unwrap();
    current_settings.retention_days = 30;
    current_settings.max_articles_per_topic = 500;

    retention::prune(&db.pool, &current_settings).await.unwrap();

    let remaining = articles::list_for_topic(&db.pool, topic.id, 10)
        .await
        .unwrap();
    assert_eq!(remaining.len(), 1);
    assert_eq!(remaining[0].title, "Recent");
}
