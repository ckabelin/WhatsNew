<script lang="ts">
  import { Search } from 'lucide-svelte';
  import EmptyState from '$lib/components/EmptyState.svelte';
  import SearchResultItem from '$lib/components/SearchResultItem.svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import Input from '$lib/components/ui/Input.svelte';
  import {
    searchQuery,
    searchResults,
    searchLoading,
    searchError,
    searchPerformed,
    performSearch
  } from '$lib/stores/search';

  async function onSubmit(event: SubmitEvent) {
    event.preventDefault();
    await performSearch($searchQuery);
  }
</script>

<div class="flex flex-col gap-4 p-4">
  <form class="flex items-center gap-2" onsubmit={onSubmit}>
    <Input
      bind:value={$searchQuery}
      type="search"
      placeholder="Search for news…"
      class="flex-1"
      aria-label="Search for news"
    />
    <Button type="submit" disabled={$searchLoading || !$searchQuery.trim()}>
      <Search size={16} />
      Search
    </Button>
  </form>

  {#if $searchLoading}
    <p class="text-sm text-text-muted">Searching…</p>
  {:else if $searchError}
    <EmptyState title="Search failed" description={$searchError} />
  {:else if $searchPerformed && $searchResults.length === 0}
    <EmptyState
      title="No results"
      description="No news articles were found for that search. Try a different query."
    />
  {:else if $searchResults.length > 0}
    <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
      {#each $searchResults as result, index (`${result.link ?? result.title}-${index}`)}
        <SearchResultItem {result} />
      {/each}
    </div>
  {:else}
    <EmptyState
      title="Search for news"
      description="Type a query above to search for recent news articles on any topic."
    />
  {/if}
</div>
