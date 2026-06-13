import {
  Briefcase,
  Car,
  Clapperboard,
  Code2,
  Cpu,
  FlaskConical,
  Gamepad2,
  GraduationCap,
  HeartPulse,
  Landmark,
  Leaf,
  Music,
  Newspaper,
  ShieldAlert,
  Trophy,
  UtensilsCrossed,
  type Icon
} from 'lucide-svelte';

export interface TopicCategory {
  label: string;
  icon: typeof Icon;
  keywords: string[];
}

/**
 * Keyword -> category rules, checked in order against the topic name. The
 * first match wins, so more specific keywords should come before broader
 * ones. Drives both the per-topic sidebar icon (topicIcon.ts) and the
 * sidebar's auto-group-by-category feature. Icons are from lucide-svelte
 * (ISC licensed, already bundled) - no external icon fetching, so no
 * per-topic licensing concerns.
 */
export const CATEGORIES: TopicCategory[] = [
  {
    label: 'Programming',
    icon: Code2,
    keywords: ['programming', 'code', 'coding', 'developer', 'open source', 'github']
  },
  {
    label: 'Technology',
    icon: Cpu,
    keywords: [
      'tech',
      'technology',
      'ai',
      'artificial intelligence',
      'software',
      'gadgets',
      'computing'
    ]
  },
  {
    label: 'Security',
    icon: ShieldAlert,
    keywords: ['security', 'cyber', 'cybersecurity', 'privacy']
  },
  { label: 'Gaming', icon: Gamepad2, keywords: ['gaming', 'game', 'games', 'esports'] },
  {
    label: 'Sports',
    icon: Trophy,
    keywords: [
      'sport',
      'sports',
      'football',
      'soccer',
      'basketball',
      'baseball',
      'olympics',
      'tennis',
      'formula 1',
      'f1',
      'racing'
    ]
  },
  {
    label: 'Science',
    icon: FlaskConical,
    keywords: ['science', 'research', 'physics', 'biology', 'chemistry', 'space', 'nasa']
  },
  {
    label: 'Health',
    icon: HeartPulse,
    keywords: ['health', 'medicine', 'medical', 'wellness', 'fitness']
  },
  {
    label: 'Environment',
    icon: Leaf,
    keywords: ['climate', 'environment', 'sustainability', 'energy', 'weather']
  },
  {
    label: 'Business',
    icon: Briefcase,
    keywords: ['business', 'startup', 'startups', 'company', 'companies']
  },
  {
    label: 'Finance',
    icon: Landmark,
    keywords: ['market', 'markets', 'finance', 'economy', 'stocks', 'crypto', 'cryptocurrency']
  },
  {
    label: 'Entertainment',
    icon: Clapperboard,
    keywords: ['entertainment', 'movie', 'movies', 'film', 'tv', 'television', 'celebrity']
  },
  { label: 'Music', icon: Music, keywords: ['music'] },
  {
    label: 'Food',
    icon: UtensilsCrossed,
    keywords: ['food', 'cooking', 'recipe', 'recipes', 'restaurant']
  },
  {
    label: 'Automotive',
    icon: Car,
    keywords: ['car', 'cars', 'automotive', 'ev', 'electric vehicle']
  },
  {
    label: 'Education',
    icon: GraduationCap,
    keywords: ['education', 'school', 'university', 'learning']
  },
  {
    label: 'Politics',
    icon: Landmark,
    keywords: ['politics', 'political', 'government', 'election']
  }
];

export const GENERAL_CATEGORY: TopicCategory = { label: 'General', icon: Newspaper, keywords: [] };

/**
 * Categorizes a topic's free-text name by keyword match, falling back to a
 * generic "General" category for anything unrecognized.
 */
export function getTopicCategory(topicName: string): TopicCategory {
  const name = topicName.toLowerCase();
  for (const category of CATEGORIES) {
    if (category.keywords.some((keyword) => name.includes(keyword))) {
      return category;
    }
  }
  return GENERAL_CATEGORY;
}
