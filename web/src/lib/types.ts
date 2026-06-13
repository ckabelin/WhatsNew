export interface Topic {
  id: number;
  name: string;
  notifications_enabled: boolean;
  initial_refresh_done: boolean;
  sort_order: number;
  created_at: string;
}

export interface Feed {
  id: number;
  url: string;
  title: string | null;
  site_url: string | null;
  etag: string | null;
  last_modified: string | null;
  last_fetched_at: string | null;
  last_error: string | null;
}

export interface Article {
  id: number;
  feed_id: number;
  guid: string;
  title: string;
  link: string | null;
  summary: string | null;
  published_at: string | null;
  fetched_at: string;
}

export interface ReadableArticle {
  article: Article;
  source_url: string;
  title: string;
  paragraphs: string[];
  images: ReadableImage[];
  content: ReadableBlock[];
}

export interface ReadableImage {
  url: string;
  alt: string | null;
}

export type ReadableBlock =
  | {
      kind: 'paragraph';
      text: string;
    }
  | {
      kind: 'image';
      image: ReadableImage;
    };

export interface Settings {
  retention_days: number;
  max_articles_per_topic: number;
  max_cache_size_mb: number;
  refresh_interval_minutes: number;
  notifications_enabled: boolean;
}

export interface DiscoveredFeed {
  url: string;
  title: string | null;
}
