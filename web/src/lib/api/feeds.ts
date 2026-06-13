import { invoke } from '@tauri-apps/api/core';
import type { DiscoveredFeed, Feed } from '$lib/types';

export const listTopicFeeds = (topicId: number) => invoke<Feed[]>('list_topic_feeds', { topicId });

export const addFeedToTopic = (topicId: number, url: string) =>
  invoke<Feed>('add_feed_to_topic', { topicId, url });

export const removeFeedFromTopic = (topicId: number, feedId: number) =>
  invoke<void>('remove_feed_from_topic', { topicId, feedId });

export const discoverFeedsForSite = (siteUrl: string) =>
  invoke<DiscoveredFeed[]>('discover_feeds_for_site', { siteUrl });
