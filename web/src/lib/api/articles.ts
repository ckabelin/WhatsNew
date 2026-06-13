import { invoke } from '@tauri-apps/api/core';
import type { Article, ReadableArticle } from '$lib/types';

export const listArticles = (topicId: number, limit = 100) =>
  invoke<Article[]>('list_articles', { topicId, limit });

export const readArticle = (articleId: number) =>
  invoke<ReadableArticle>('read_article', { articleId });

export const refreshTopicNow = (topicId: number) =>
  invoke<number>('refresh_topic_now', { topicId });
