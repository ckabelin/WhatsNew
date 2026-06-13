<script lang="ts">
  import { page } from '$app/stores';
  import { RefreshCw } from 'lucide-svelte';
  import ArticleCard from '$lib/components/ArticleCard.svelte';
  import EmptyState from '$lib/components/EmptyState.svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import { articles, articlesLoading, loadArticles, refreshAndReload } from '$lib/stores/articles';

  let topicId = $derived(Number($page.params.id));
  let refreshing = $state(false);

  $effect(() => {
    loadArticles(topicId);
  });

  async function onRefresh() {
    refreshing = true;
    try {
      await refreshAndReload(topicId);
    } finally {
      refreshing = false;
    }
  }
</script>

<div class="flex flex-col gap-4 p-4">
  <div class="flex items-center justify-end">
    <Button variant="secondary" onclick={onRefresh} disabled={refreshing}>
      <RefreshCw size={16} class={refreshing ? 'animate-spin' : ''} />
      Refresh
    </Button>
  </div>

  {#if $articlesLoading}
    <p class="text-sm text-neutral-400">Loading articles…</p>
  {:else if $articles.length === 0}
    <EmptyState
      title="No articles yet"
      description="Refresh to fetch the latest articles for this topic."
    />
  {:else}
    <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
      {#each $articles as article (article.id)}
        <ArticleCard {article} />
      {/each}
    </div>
  {/if}
</div>
