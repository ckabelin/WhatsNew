import { writable } from 'svelte/store';
import { load, type Store } from '@tauri-apps/plugin-store';

export type ViewMode = 'grid' | 'list' | 'magazine' | 'headlines';
export type SortMode = 'newest' | 'oldest' | 'relevancy';

export const viewMode = writable<ViewMode>('grid');
export const sortMode = writable<SortMode>('newest');
export const sidebarWidth = writable<number>(224);
export const autoGroupTopics = writable<boolean>(false);
export const expandedTopicGroup = writable<string | null>(null);
export const topicGroupOrder = writable<string[]>([]);

const STORE_FILE = 'view-preferences.json';
let store: Store | undefined;

export async function initViewPreferences() {
  if (store) return;
  store = await load(STORE_FILE);

  const savedView = await store.get<ViewMode>('viewMode');
  const savedSort = await store.get<SortMode>('sortMode');
  const savedSidebarWidth = await store.get<number>('sidebarWidth');
  const savedAutoGroupTopics = await store.get<boolean>('autoGroupTopics');
  const savedExpandedTopicGroup = await store.get<string | null>('expandedTopicGroup');
  const savedTopicGroupOrder = await store.get<string[]>('topicGroupOrder');

  if (savedView) viewMode.set(savedView);
  if (savedSort) sortMode.set(savedSort);
  if (savedSidebarWidth) sidebarWidth.set(savedSidebarWidth);
  if (savedAutoGroupTopics !== undefined) autoGroupTopics.set(savedAutoGroupTopics);
  if (savedExpandedTopicGroup !== undefined) expandedTopicGroup.set(savedExpandedTopicGroup);
  if (savedTopicGroupOrder) topicGroupOrder.set(savedTopicGroupOrder);

  viewMode.subscribe((value) => void store?.set('viewMode', value));
  sortMode.subscribe((value) => void store?.set('sortMode', value));
  sidebarWidth.subscribe((value) => void store?.set('sidebarWidth', value));
  autoGroupTopics.subscribe((value) => void store?.set('autoGroupTopics', value));
  expandedTopicGroup.subscribe((value) => void store?.set('expandedTopicGroup', value));
  topicGroupOrder.subscribe((value) => void store?.set('topicGroupOrder', value));
}
