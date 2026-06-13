import { type Icon } from 'lucide-svelte';
import { getTopicCategory } from '$lib/topicCategory';

/**
 * Picks a topical lucide icon for a topic's free-text name by keyword match,
 * falling back to a generic newspaper icon for anything unrecognized.
 */
export function getTopicIcon(topicName: string): typeof Icon {
  return getTopicCategory(topicName).icon;
}
