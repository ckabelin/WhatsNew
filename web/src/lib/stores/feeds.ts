import { writable } from 'svelte/store';
import type { Feed } from '$lib/types';
import * as feedsApi from '$lib/api/feeds';

export const topicFeeds = writable<Feed[]>([]);

export async function loadTopicFeeds(topicId: number) {
  topicFeeds.set(await feedsApi.listTopicFeeds(topicId));
}
