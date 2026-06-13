<script lang="ts">
  import { openUrl } from '@tauri-apps/plugin-opener';
  import { ArrowLeft } from 'lucide-svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import { readSearchResult } from '$lib/api/search';
  import { searchReadTarget } from '$lib/stores/search';
  import type { ReadableUrl } from '$lib/types';

  let readable = $state<ReadableUrl | null>(null);
  let loading = $state(true);
  let error = $state<string | null>(null);

  $effect(() => {
    const target = $searchReadTarget;
    loadResult(target);
  });

  async function loadResult(target: { url: string; title: string } | null) {
    loading = true;
    error = null;
    readable = null;

    if (!target) {
      error = 'No article selected. Go back and pick a search result.';
      loading = false;
      return;
    }

    try {
      readable = await readSearchResult(target.url, target.title);
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
      <span class="min-w-0 truncate text-sm font-medium text-text-muted">{readable.title}</span>
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
    {:else}
      <p class="text-sm text-text-muted">No readable article body was found.</p>
    {/if}
  {/if}
</article>
