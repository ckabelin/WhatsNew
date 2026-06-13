import { writable } from 'svelte/store';
import type { Topic } from '$lib/types';
import * as topicsApi from '$lib/api/topics';

export const topics = writable<Topic[]>([]);

export async function loadTopics() {
  topics.set(await topicsApi.listTopics());
}

export async function addTopic(name: string) {
  const topic = await topicsApi.createTopic(name);
  topics.update((list) => [...list, topic].sort((a, b) => a.sort_order - b.sort_order));
  return topic;
}

export async function removeTopic(id: number) {
  await topicsApi.deleteTopic(id);
  topics.update((list) => list.filter((t) => t.id !== id));
}

export async function renameTopicInStore(id: number, name: string) {
  const updated = await topicsApi.renameTopic(id, name);
  topics.update((list) => list.map((t) => (t.id === id ? updated : t)));
}

export async function reorderTopics(topicIds: number[]) {
  topics.set(await topicsApi.reorderTopics(topicIds));
}

export async function setTopicNotifications(id: number, enabled: boolean) {
  const updated = await topicsApi.setTopicNotifications(id, enabled);
  topics.update((list) => list.map((t) => (t.id === id ? updated : t)));
}
