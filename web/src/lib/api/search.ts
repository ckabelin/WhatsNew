import { invoke } from '@tauri-apps/api/core';
import type { ReadableUrl, SearchArticle } from '$lib/types';

export const searchNews = (query: string) => invoke<SearchArticle[]>('search_news', { query });

export const readSearchResult = (url: string, title: string) =>
  invoke<ReadableUrl>('read_search_result', { url, title });
