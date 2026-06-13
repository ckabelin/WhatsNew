import type { Article } from '$lib/types';
import type { SortMode } from '$lib/stores/viewPreferences';
import { relevanceScore } from '$lib/relevance';

function timeValue(article: Article): number {
  const time = new Date(article.published_at ?? article.fetched_at).getTime();
  return Number.isNaN(time) ? 0 : time;
}

export function sortArticles(
  articles: Article[],
  sortMode: SortMode,
  topicName: string
): Article[] {
  const sorted = [...articles];

  switch (sortMode) {
    case 'oldest':
      sorted.sort((a, b) => timeValue(a) - timeValue(b));
      break;
    case 'relevancy':
      sorted.sort((a, b) => {
        const diff = relevanceScore(topicName, b) - relevanceScore(topicName, a);
        return diff !== 0 ? diff : timeValue(b) - timeValue(a);
      });
      break;
    case 'newest':
    default:
      sorted.sort((a, b) => timeValue(b) - timeValue(a));
      break;
  }

  return sorted;
}
