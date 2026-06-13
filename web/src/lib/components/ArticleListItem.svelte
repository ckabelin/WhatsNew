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
  class="flex items-center gap-2 rounded-lg border border-border bg-surface p-3 transition-colors hover:border-border-strong hover:bg-surface-hover"
>
  <a
    href={resolve('/article/[id]', { id: String(article.id) })}
    class="flex min-w-0 flex-1 flex-col gap-1 text-left sm:flex-row sm:items-center sm:justify-between sm:gap-4"
  >
    <div class="flex flex-col gap-1 sm:min-w-0">
      <h3 class="text-sm font-semibold text-text">{article.title}</h3>
      {#if article.summary}
        <p class="line-clamp-1 text-xs text-text-muted">{article.summary}</p>
      {/if}
    </div>
    {#if published}
      <span class="shrink-0 text-xs text-text-subtle">{published}</span>
    {/if}
  </a>
  <BookmarkButton {article} onToggle={onToggleFavorite} class="shrink-0" />
</div>
