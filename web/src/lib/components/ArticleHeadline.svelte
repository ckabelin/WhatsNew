<script lang="ts">
  import { resolve } from '$app/paths';

  import BookmarkButton from '$lib/components/BookmarkButton.svelte';
  import { formatDateTime } from '$lib/format';
  import type { Article } from '$lib/types';

  interface Props {
    article: Article;
    onToggleFavorite: (article: Article) => unknown;
  }

  let { article, onToggleFavorite }: Props = $props();

  const published = $derived(formatDateTime(article.published_at));
</script>

<div
  class="flex items-center gap-2 rounded-md px-2 py-1.5 transition-colors hover:bg-surface-hover"
>
  <a
    href={resolve('/article/[id]', { id: String(article.id) })}
    class="flex min-w-0 flex-1 items-baseline justify-between gap-4 text-left"
  >
    <h3 class="truncate text-sm text-text">{article.title}</h3>
    {#if published}
      <span class="shrink-0 text-xs text-text-subtle">{published}</span>
    {/if}
  </a>
  <BookmarkButton {article} onToggle={onToggleFavorite} class="shrink-0" />
</div>
