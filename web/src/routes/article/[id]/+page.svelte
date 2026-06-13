<script lang="ts">
  import { page } from '$app/stores';
  import { openUrl } from '@tauri-apps/plugin-opener';
  import { ArrowLeft } from 'lucide-svelte';
  import BookmarkButton from '$lib/components/BookmarkButton.svelte';
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

  function openSourceUrl(url: string) {
    void openUrl(url);
  }

  async function onToggleFavorite(article: ReadableArticle['article']) {
    const updated = await articlesApi.setArticleFavorite(article.id, !article.is_favorite);
    if (readable) {
      readable = { ...readable, article: updated };
    }
    return updated;
  }
</script>

<article class="mx-auto flex max-w-3xl flex-col gap-6 p-6">
  <div
    class="sticky top-0 z-10 -mx-6 -mt-6 flex items-center gap-3 border-b border-border bg-bg/95 px-6 py-3"
  >
    <Button variant="ghost" class="shrink-0" onclick={goBack}>
      <ArrowLeft size={16} />
      Back
    </Button>
    {#if readable}
      <span class="min-w-0 flex-1 truncate text-sm font-medium text-text-muted"
        >{readable.title}</span
      >
      <BookmarkButton article={readable.article} onToggle={onToggleFavorite} class="shrink-0" />
    {/if}
  </div>

  {#if loading}
    <p class="text-sm text-text-muted">Loading article...</p>
  {:else if error}
    <div class="rounded-lg border border-error-border/60 bg-error-bg p-4">
      <h1 class="text-lg font-semibold text-error-heading">Article could not be loaded</h1>
      <p class="mt-2 text-sm text-error-text">{error}</p>
    </div>
  {:else if readable}
    <header class="border-b border-border pb-5">
      <button
        type="button"
        class="mb-2 inline-block break-all text-left text-xs text-text-subtle hover:text-accent hover:underline"
        onclick={() => openSourceUrl(readable!.source_url)}
      >
        {readable.source_url}
      </button>
      <h1 class="text-2xl font-semibold leading-tight text-text">{readable.title}</h1>
      {#if published}
        <p class="mt-3 text-sm text-text-subtle">Published {published}</p>
      {/if}
    </header>

    {#if readable.content.length > 0}
      <div class="flex flex-col gap-4 text-base leading-7 text-text-muted">
        {#each readable.content as block, index (`${block.kind}-${index}`)}
          {#if block.kind === 'paragraph'}
            <p>{block.text}</p>
          {:else}
            <figure
              class="my-2 overflow-hidden rounded-lg border border-border bg-surface leading-normal"
            >
              <img
                src={block.image.url}
                alt={block.image.alt ?? ''}
                class="max-h-[520px] w-full object-contain"
                loading="lazy"
                referrerpolicy="no-referrer"
              />
              {#if block.image.alt}
                <figcaption class="border-t border-border px-3 py-2 text-xs text-text-subtle">
                  {block.image.alt}
                </figcaption>
              {/if}
            </figure>
          {/if}
        {/each}
      </div>
    {:else if readable.article.summary}
      <div class="flex flex-col gap-4 text-base leading-7 text-text-muted">
        <p>{readable.article.summary}</p>
      </div>
    {:else}
      <p class="text-sm text-text-muted">No readable article body was found.</p>
    {/if}
  {/if}
</article>
