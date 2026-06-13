import { invoke } from '@tauri-apps/api/core';
import type { Article } from '$lib/types';

export const listArticles = (topicId: number, limit = 100) =>
  invoke<Article[]>('list_articles', { topicId, limit });

export const refreshTopicNow = (topicId: number) =>
  invoke<number>('refresh_topic_now', { topicId });
