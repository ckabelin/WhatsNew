<script lang="ts">
  import type { Article } from '$lib/types';

  interface Props {
    article: Article;
  }

  let { article }: Props = $props();

  function formatDate(value: string | null): string | null {
    if (!value) return null;
    const date = new Date(value);
    if (Number.isNaN(date.getTime())) return null;
    return date.toLocaleDateString(undefined, { year: 'numeric', month: 'short', day: 'numeric' });
  }

  const published = $derived(formatDate(article.published_at));
</script>

<!-- eslint-disable-next-line svelte/no-navigation-without-resolve -->
<a
  href={article.link ?? undefined}
  data-sveltekit-reload
  target="_blank"
  rel="noreferrer noopener"
  class="flex flex-col gap-2 rounded-lg border border-neutral-800 bg-neutral-900 p-4 transition-colors hover:border-neutral-700 hover:bg-neutral-800"
>
  <h3 class="text-sm font-semibold text-neutral-100">{article.title}</h3>
  {#if article.summary}
    <p class="line-clamp-3 text-sm text-neutral-400">{article.summary}</p>
  {/if}
  {#if published}
    <span class="mt-auto text-xs text-neutral-500">{published}</span>
  {/if}
</a>
