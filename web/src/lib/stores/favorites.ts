import { writable } from 'svelte/store';
import type { Article } from '$lib/types';
import * as articlesApi from '$lib/api/articles';

export const favoriteArticles = writable<Article[]>([]);
export const favoriteArticlesLoading = writable(false);

export async function loadFavoriteArticles() {
  favoriteArticlesLoading.set(true);
  try {
    favoriteArticles.set(await articlesApi.listFavoriteArticles());
  } finally {
    favoriteArticlesLoading.set(false);
  }
}

// Toggles `article`'s favorite flag and removes it from the list if it was
// just unfavorited, since the favorites page only shows favorited articles.
export async function toggleFavorite(article: Article) {
  const updated = await articlesApi.setArticleFavorite(article.id, !article.is_favorite);
  favoriteArticles.update((list) =>
    updated.is_favorite
      ? list.map((a) => (a.id === updated.id ? updated : a))
      : list.filter((a) => a.id !== updated.id)
  );
  return updated;
}
