import { get, writable } from 'svelte/store';
import type { SearchArticle } from '$lib/types';
import * as searchApi from '$lib/api/search';

export interface SearchReadTarget {
  url: string;
  title: string;
}

export const searchReadTarget = writable<SearchReadTarget | null>(null);

export const searchQuery = writable('');
export const searchResults = writable<SearchArticle[]>([]);
export const searchLoading = writable(false);
export const searchError = writable<string | null>(null);
export const searchPerformed = writable(false);

const AUTO_REFRESH_INTERVAL_MS = 60_000;
let refreshTimer: ReturnType<typeof setInterval> | null = null;

export async function performSearch(query: string) {
  const trimmed = query.trim();
  if (!trimmed) return;

  searchQuery.set(trimmed);
  searchLoading.set(true);
  searchError.set(null);
  try {
    searchResults.set(await searchApi.searchNews(trimmed));
    searchPerformed.set(true);
    startAutoRefresh();
  } catch (err) {
    searchError.set(err instanceof Error ? err.message : String(err));
    searchResults.set([]);
    stopAutoRefresh();
  } finally {
    searchLoading.set(false);
  }
}

async function refreshSearchResults() {
  const query = get(searchQuery);
  if (!query) return;

  try {
    const results = await searchApi.searchNews(query);
    searchResults.set(results);
    searchError.set(null);
  } catch {
    // Keep showing the previous results if a background refresh fails.
  }
}

function startAutoRefresh() {
  if (refreshTimer) return;
  refreshTimer = setInterval(refreshSearchResults, AUTO_REFRESH_INTERVAL_MS);
}

function stopAutoRefresh() {
  if (refreshTimer) {
    clearInterval(refreshTimer);
    refreshTimer = null;
  }
}

export function clearSearch() {
  searchQuery.set('');
  searchResults.set([]);
  searchError.set(null);
  searchPerformed.set(false);
  stopAutoRefresh();
}
