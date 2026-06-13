<script lang="ts">
  import { resolve } from '$app/paths';

  import BookmarkButton from '$lib/components/BookmarkButton.svelte';
  import { formatDateTime } from '$lib/format';
  import type { Article } from '$lib/types';

  interface Props {
    article: Article;
    featured?: boolean;
    onToggleFavorite: (article: Article) => unknown;
  }

  let { article, featured = false, onToggleFavorite }: Props = $props();

  const published = $derived(formatDateTime(article.published_at));
</script>

<div class="relative">
  <a
    href={resolve('/article/[id]', { id: String(article.id) })}
    class="flex flex-col gap-2 rounded-lg border border-border bg-surface text-left transition-colors hover:border-border-strong hover:bg-surface-hover
      {featured ? 'p-6' : 'p-4'}"
  >
    <h3 class="pr-8 font-semibold text-text {featured ? 'text-lg' : 'text-sm'}">
      {article.title}
    </h3>
    {#if article.summary}
      <p class="text-text-muted {featured ? 'line-clamp-5 text-base' : 'line-clamp-3 text-sm'}">
        {article.summary}
      </p>
    {/if}
    {#if published}
      <span class="mt-auto text-xs text-text-subtle">{published}</span>
    {/if}
  </a>
  <BookmarkButton
    {article}
    onToggle={onToggleFavorite}
    class="absolute right-2 top-2 bg-surface/80"
  />
</div>
