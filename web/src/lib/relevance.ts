import type { Article } from '$lib/types';

const STOP_WORDS = new Set([
  'the',
  'a',
  'an',
  'and',
  'or',
  'of',
  'for',
  'to',
  'in',
  'on',
  'with',
  'news'
]);

function extractKeywords(topicName: string): string[] {
  return topicName
    .toLowerCase()
    .split(/[^a-z0-9]+/)
    .filter((word) => word.length > 1 && !STOP_WORDS.has(word));
}

/**
 * Scores an article's relevance to a topic by how many of the topic's
 * significant words appear in its title (weighted higher) or summary.
 */
export function relevanceScore(topicName: string, article: Article): number {
  const keywords = extractKeywords(topicName);
  if (keywords.length === 0) return 0;

  const title = article.title.toLowerCase();
  const summary = article.summary?.toLowerCase() ?? '';

  let score = 0;
  for (const keyword of keywords) {
    if (title.includes(keyword)) {
      score += 2;
    } else if (summary.includes(keyword)) {
      score += 1;
    }
  }
  return score;
}
