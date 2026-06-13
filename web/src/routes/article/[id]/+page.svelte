<script lang="ts">
  import { page } from '$app/stores';
  import { ArrowLeft } from 'lucide-svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import * as articlesApi from '$lib/api/articles';
  import { formatDateTime } from '$lib/format';
  import type { ReadableArticle } from '$lib/types';

  let articleId = $derived(Number($page.params.id));
  let readable = $state<ReadableArticle | null>(null);
  let loading = $state(true);
  let error = $state<string | null>(null);
  let published = $derived(formatDateTime(readable?.article.published_at ?? null));

  $effect(() => {
    loadArticle(articleId);
  });

  async function loadArticle(id: number) {
    loading = true;
    error = null;
    readable = null;

    try {
      readable = await articlesApi.readArticle(id);
    } catch (err) {
      error = err instanceof Error ? err.message : String(err);
    } finally {
      loading = false;
    }
  }

  function goBack() {
    history.back();
  }
</script>

<article class="mx-auto flex max-w-3xl flex-col gap-6 p-6">
  <div
    class="sticky top-0 z-10 -mx-6 -mt-6 flex items-center gap-3 border-b border-neutral-800 bg-neutral-950/95 px-6 py-3"
  >
    <Button variant="ghost" class="shrink-0" onclick={goBack}>
      <ArrowLeft size={16} />
      Back
    </Button>
    {#if readable}
      <span class="min-w-0 truncate text-sm font-medium text-neutral-300">{readable.title}</span>
    {/if}
  </div>

  {#if loading}
    <p class="text-sm text-neutral-400">Loading article...</p>
  {:else if error}
    <div class="rounded-lg border border-red-900/60 bg-red-950/30 p-4">
      <h1 class="text-lg font-semibold text-red-100">Article could not be loaded</h1>
      <p class="mt-2 text-sm text-red-200">{error}</p>
    </div>
  {:else if readable}
    <header class="border-b border-neutral-800 pb-5">
      <p class="mb-2 break-all text-xs text-neutral-500">{readable.source_url}</p>
      <h1 class="text-2xl font-semibold leading-tight text-neutral-100">{readable.title}</h1>
      {#if published}
        <p class="mt-3 text-sm text-neutral-500">Published {published}</p>
      {/if}
    </header>

    {#if readable.content.length > 0}
      <div class="flex flex-col gap-4 text-base leading-7 text-neutral-200">
        {#each readable.content as block, index (`${block.kind}-${index}`)}
          {#if block.kind === 'paragraph'}
            <p>{block.text}</p>
          {:else}
            <figure
              class="my-2 overflow-hidden rounded-lg border border-neutral-800 bg-neutral-900 leading-normal"
            >
              <img
                src={block.image.url}
                alt={block.image.alt ?? ''}
                class="max-h-[520px] w-full object-contain"
                loading="lazy"
                referrerpolicy="no-referrer"
              />
              {#if block.image.alt}
                <figcaption class="border-t border-neutral-800 px-3 py-2 text-xs text-neutral-500">
                  {block.image.alt}
                </figcaption>
              {/if}
            </figure>
          {/if}
        {/each}
      </div>
    {:else if readable.article.summary}
      <div class="flex flex-col gap-4 text-base leading-7 text-neutral-200">
        <p>{readable.article.summary}</p>
      </div>
    {:else}
      <p class="text-sm text-neutral-400">No readable article body was found.</p>
    {/if}
  {/if}
</article>
