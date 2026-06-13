import { writable } from 'svelte/store';
import type { Article } from '$lib/types';
import * as articlesApi from '$lib/api/articles';

export const articles = writable<Article[]>([]);
export const articlesLoading = writable(false);

export async function loadArticles(topicId: number) {
  articlesLoading.set(true);
  try {
    articles.set(await articlesApi.listArticles(topicId));
  } finally {
    articlesLoading.set(false);
  }
}

export async function refreshAndReload(topicId: number) {
  await articlesApi.refreshTopicNow(topicId);
  await loadArticles(topicId);
}
