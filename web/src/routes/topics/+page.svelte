<script lang="ts">
  import { onMount } from 'svelte';
  import Input from '$lib/components/ui/Input.svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import TopicListItem from '$lib/components/TopicListItem.svelte';
  import { topics, loadTopics, addTopic } from '$lib/stores/topics';

  let newTopicName = $state('');

  onMount(() => {
    loadTopics();
  });

  async function onAdd() {
    const name = newTopicName.trim();
    if (!name) return;
    await addTopic(name);
    newTopicName = '';
  }

  function onKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter') onAdd();
  }
</script>

<div class="flex flex-col gap-4 p-4">
  <h1 class="text-lg font-semibold text-neutral-100">Topics</h1>

  <div class="flex items-end gap-2">
    <div class="flex-1">
      <Input
        bind:value={newTopicName}
        placeholder="e.g. Artificial Intelligence"
        onkeydown={onKeydown}
      />
    </div>
    <Button onclick={onAdd}>Add</Button>
  </div>

  <div class="flex flex-col gap-2">
    {#each $topics as topic (topic.id)}
      <TopicListItem {topic} />
    {/each}
  </div>
</div>
