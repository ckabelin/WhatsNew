import { invoke } from '@tauri-apps/api/core';
import type { Topic } from '$lib/types';

export const listTopics = () => invoke<Topic[]>('list_topics');
export const createTopic = (name: string) => invoke<Topic>('create_topic', { name });
export const renameTopic = (id: number, name: string) =>
  invoke<Topic>('rename_topic', { id, name });
export const reorderTopics = (topicIds: number[]) =>
  invoke<Topic[]>('reorder_topics', { topicIds });
export const deleteTopic = (id: number) => invoke<void>('delete_topic', { id });
export const setTopicNotifications = (id: number, enabled: boolean) =>
  invoke<Topic>('set_topic_notifications', { id, enabled });
