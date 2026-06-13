-- Persist user-defined topic ordering for the sidebar.

ALTER TABLE topics ADD COLUMN sort_order INTEGER NOT NULL DEFAULT 0;

UPDATE topics
SET sort_order = id
WHERE sort_order = 0;

CREATE INDEX idx_topics_sort_order ON topics (sort_order, name COLLATE NOCASE);
