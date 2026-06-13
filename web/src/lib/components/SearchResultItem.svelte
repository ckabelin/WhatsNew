<script lang="ts">
  import { goto } from '$app/navigation';
  import { resolve } from '$app/paths';

  import { formatDateTime } from '$lib/format';
  import { searchReadTarget } from '$lib/stores/search';
  import type { SearchArticle } from '$lib/types';

  interface Props {
    result: SearchArticle;
  }

  let { result }: Props = $props();

  const published = $derived(formatDateTime(result.published_at));

  function openResult(event: MouseEvent) {
    event.preventDefault();
    if (!result.link) return;
    searchReadTarget.set({ url: result.link, title: result.title });
    goto(resolve('/search/read'));
  }
</script>

{#if result.link}
  <a
    href={resolve('/search/read')}
    onclick={openResult}
    class="flex flex-col gap-2 rounded-lg border border-border bg-surface p-4 text-left transition-colors hover:border-border-strong hover:bg-surface-hover"
  >
    <h3 class="text-sm font-semibold text-text">{result.title}</h3>
    {#if result.summary}
      <p class="line-clamp-3 text-sm text-text-muted">{result.summary}</p>
    {/if}
    {#if published}
      <span class="text-xs text-text-subtle">{published}</span>
    {/if}
  </a>
{:else}
  <div class="flex flex-col gap-2 rounded-lg border border-border bg-surface p-4">
    <h3 class="text-sm font-semibold text-text">{result.title}</h3>
    {#if result.summary}
      <p class="line-clamp-3 text-sm text-text-muted">{result.summary}</p>
    {/if}
    {#if published}
      <span class="text-xs text-text-subtle">{published}</span>
    {/if}
  </div>
{/if}
