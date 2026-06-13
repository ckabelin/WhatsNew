<script lang="ts">
  import { resolve } from '$app/paths';
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import EmptyState from '$lib/components/EmptyState.svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import { topics, loadTopics } from '$lib/stores/topics';

  let loading = $state(true);

  onMount(async () => {
    await loadTopics();
    const list = $topics;
    if (list.length > 0) {
      await goto(resolve('/topic/[id]', { id: String(list[0].id) }));
    } else {
      loading = false;
    }
  });
</script>

{#if !loading}
  <EmptyState title="No topics yet" description="Add a topic to start aggregating news for it.">
    {#snippet action()}
      <Button onclick={() => goto(resolve('/topics'))}>Add a topic</Button>
    {/snippet}
  </EmptyState>
{/if}
