<script lang="ts">
  import { resolve } from '$app/paths';

  import { formatDateTime } from '$lib/format';
  import type { Article } from '$lib/types';

  interface Props {
    article: Article;
  }

  let { article }: Props = $props();

  const published = $derived(formatDateTime(article.published_at));
</script>

<a
  href={resolve('/article/[id]', { id: String(article.id) })}
  class="flex flex-col gap-2 rounded-lg border border-neutral-800 bg-neutral-900 p-4 text-left transition-colors hover:border-neutral-700 hover:bg-neutral-800"
>
  <h3 class="text-sm font-semibold text-neutral-100">{article.title}</h3>
  {#if article.summary}
    <p class="line-clamp-3 text-sm text-neutral-400">{article.summary}</p>
  {/if}
  {#if published}
    <span class="mt-auto text-xs text-neutral-500">{published}</span>
  {/if}
</a>
