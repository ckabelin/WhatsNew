-- Allow articles to be bookmarked/favorited, independent of their topic(s).

ALTER TABLE articles ADD COLUMN is_favorite INTEGER NOT NULL DEFAULT 0;

CREATE INDEX idx_articles_favorite ON articles (is_favorite) WHERE is_favorite = 1;
