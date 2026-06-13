<script lang="ts">
  import { Bookmark } from 'lucide-svelte';
  import type { Article } from '$lib/types';

  interface Props {
    article: Article;
    onToggle: (article: Article) => unknown;
    class?: string;
  }

  let { article, onToggle, class: className = '' }: Props = $props();
  let toggling = $state(false);

  async function handleClick(event: MouseEvent) {
    event.preventDefault();
    event.stopPropagation();
    if (toggling) return;
    toggling = true;
    try {
      await onToggle(article);
    } finally {
      toggling = false;
    }
  }
</script>

<button
  type="button"
  disabled={toggling}
  onclick={handleClick}
  aria-label={article.is_favorite ? 'Remove bookmark' : 'Add bookmark'}
  aria-pressed={article.is_favorite}
  class="inline-flex items-center justify-center rounded-md p-1.5 transition-colors disabled:opacity-50
    {article.is_favorite
    ? 'text-favorite hover:text-favorite-hover'
    : 'text-text-subtle hover:bg-surface-hover hover:text-text-muted'} {className}"
>
  <Bookmark size={16} fill={article.is_favorite ? 'currentColor' : 'none'} />
</button>
