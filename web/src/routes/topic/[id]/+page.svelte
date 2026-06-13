<script lang="ts">
  import { page } from '$app/stores';
  import { RefreshCw } from 'lucide-svelte';
  import ArticleCard from '$lib/components/ArticleCard.svelte';
  import ArticleHeadline from '$lib/components/ArticleHeadline.svelte';
  import ArticleListItem from '$lib/components/ArticleListItem.svelte';
  import EmptyState from '$lib/components/EmptyState.svelte';
  import ViewControls from '$lib/components/ViewControls.svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import {
    articles,
    articlesLoading,
    loadArticles,
    refreshAndReload,
    toggleArticleFavorite
  } from '$lib/stores/articles';
  import { topicFeeds, loadTopicFeeds } from '$lib/stores/feeds';
  import { topics } from '$lib/stores/topics';
  import { viewMode, sortMode } from '$lib/stores/viewPreferences';
  import { sortArticles } from '$lib/sort';

  let topicId = $derived(Number($page.params.id));
  let refreshing = $state(false);

  let topic = $derived($topics.find((t) => t.id === topicId));
  let topicName = $derived(topic?.name ?? '');
  let sortedArticles = $derived(sortArticles($articles, $sortMode, topicName));

  let emptyState = $derived.by(() => {
    if (!topic?.initial_refresh_done) {
      return {
        title: 'Fetching articles…',
        description:
          'WhatsNew is loading articles for this topic for the first time. This can take a moment - click Refresh if nothing appears.'
      };
    }

    if ($topicFeeds.length === 0) {
      return {
        title: 'No feeds for this topic',
        description: 'No feeds are linked to this topic yet. Try refreshing to find some.'
      };
    }

    const errors = $topicFeeds
      .filter((feed) => feed.last_error)
      .map((feed) => `${feed.title ?? feed.url}: ${feed.last_error}`);

    if (errors.length === $topicFeeds.length) {
      return {
        title: 'Couldn’t load any feeds',
        description: `Every feed for this topic failed to load.\n${errors.join('\n')}`
      };
    }

    if (errors.length > 0) {
      return {
        title: 'No articles yet',
        description: `Some feeds returned no articles, and some failed:\n${errors.join('\n')}`
      };
    }

    return {
      title: 'No articles yet',
      description: 'No recent articles were found for this topic. Try refreshing again later.'
    };
  });

  $effect(() => {
    loadArticles(topicId);
    loadTopicFeeds(topicId);
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
  <div class="flex flex-wrap items-center justify-between gap-3">
    <ViewControls />
    <h1 class="min-w-0 flex-1 truncate text-center text-sm font-semibold text-text">
      {topicName}
    </h1>
    <Button variant="secondary" onclick={onRefresh} disabled={refreshing}>
      <RefreshCw size={16} class={refreshing ? 'animate-spin' : ''} />
      Refresh
    </Button>
  </div>

  {#if $articlesLoading}
    <p class="text-sm text-text-muted">Loading articles…</p>
  {:else if sortedArticles.length === 0}
    <EmptyState title={emptyState.title} description={emptyState.description} />
  {:else if $viewMode === 'grid'}
    <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
      {#each sortedArticles as article (article.id)}
        <ArticleCard {article} onToggleFavorite={toggleArticleFavorite} />
      {/each}
    </div>
  {:else if $viewMode === 'list'}
    <div class="flex flex-col gap-2">
      {#each sortedArticles as article (article.id)}
        <ArticleListItem {article} onToggleFavorite={toggleArticleFavorite} />
      {/each}
    </div>
  {:else if $viewMode === 'magazine'}
    <div class="flex flex-col gap-4">
      <ArticleCard article={sortedArticles[0]} featured onToggleFavorite={toggleArticleFavorite} />
      <div class="flex flex-col gap-2">
        {#each sortedArticles.slice(1) as article (article.id)}
          <ArticleListItem {article} onToggleFavorite={toggleArticleFavorite} />
        {/each}
      </div>
    </div>
  {:else}
    <div class="flex flex-col">
      {#each sortedArticles as article (article.id)}
        <ArticleHeadline {article} onToggleFavorite={toggleArticleFavorite} />
      {/each}
    </div>
  {/if}
</div>
