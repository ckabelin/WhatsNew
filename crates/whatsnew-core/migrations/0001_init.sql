-- Initial schema for WhatsNew.
-- Migrations are append-only: never edit this file after it has been merged.

CREATE TABLE topics (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    notifications_enabled INTEGER NOT NULL DEFAULT 1,
    initial_refresh_done INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE feeds (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    url TEXT NOT NULL UNIQUE,
    title TEXT,
    site_url TEXT,
    etag TEXT,
    last_modified TEXT,
    last_fetched_at TEXT,
    last_error TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE topic_feeds (
    topic_id INTEGER NOT NULL REFERENCES topics (id) ON DELETE CASCADE,
    feed_id INTEGER NOT NULL REFERENCES feeds (id) ON DELETE CASCADE,
    PRIMARY KEY (topic_id, feed_id)
);

CREATE TABLE articles (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    feed_id INTEGER NOT NULL REFERENCES feeds (id) ON DELETE CASCADE,
    guid TEXT NOT NULL,
    title TEXT NOT NULL,
    link TEXT,
    summary TEXT,
    published_at TEXT,
    fetched_at TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE (feed_id, guid)
);

CREATE INDEX idx_articles_feed_published ON articles (feed_id, published_at DESC);

CREATE TABLE settings (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    retention_days INTEGER NOT NULL DEFAULT 30,
    max_articles_per_topic INTEGER NOT NULL DEFAULT 500,
    max_cache_size_mb INTEGER NOT NULL DEFAULT 200,
    refresh_interval_minutes INTEGER NOT NULL DEFAULT 60,
    notifications_enabled INTEGER NOT NULL DEFAULT 1
);

INSERT INTO settings (id) VALUES (1);
