<script lang="ts">
  import ArticleCard from '$lib/components/ArticleCard.svelte';
  import EmptyState from '$lib/components/EmptyState.svelte';
  import {
    favoriteArticles,
    favoriteArticlesLoading,
    loadFavoriteArticles,
    toggleFavorite
  } from '$lib/stores/favorites';

  $effect(() => {
    loadFavoriteArticles();
  });
</script>

<div class="flex flex-col gap-4 p-4">
  {#if $favoriteArticlesLoading}
    <p class="text-sm text-text-muted">Loading bookmarks…</p>
  {:else if $favoriteArticles.length === 0}
    <EmptyState
      title="No bookmarks yet"
      description="Bookmark articles from any topic by clicking the bookmark icon - they'll show up here, across all topics."
    />
  {:else}
    <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
      {#each $favoriteArticles as article (article.id)}
        <ArticleCard {article} onToggleFavorite={toggleFavorite} />
      {/each}
    </div>
  {/if}
</div>
