<script lang="ts">
  import { onMount } from 'svelte';
  import Input from '$lib/components/ui/Input.svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import TopicListItem from '$lib/components/TopicListItem.svelte';
  import { topics, loadTopics, addTopic } from '$lib/stores/topics';

  let newTopicName = $state('');

  const suggestedTopics = [
    'Artificial Intelligence',
    'Technology',
    'World News',
    'Business',
    'Markets',
    'Science',
    'Health',
    'Climate',
    'Cybersecurity',
    'Politics',
    'Sports',
    'Entertainment',
    'Gaming',
    'Startups',
    'Rust Programming',
    'Open Source'
  ];

  const existingTopicNames = $derived(
    new Set($topics.map((topic) => topic.name.trim().toLocaleLowerCase()))
  );

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

  function selectSuggestedTopic(name: string) {
    newTopicName = name;
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

  <section class="flex flex-col gap-2">
    <h2 class="text-sm font-medium text-neutral-300">Popular topics</h2>
    <div class="flex flex-wrap gap-2">
      {#each suggestedTopics as topic (topic)}
        {@const exists = existingTopicNames.has(topic.toLocaleLowerCase())}
        <button
          type="button"
          class="rounded-full border border-neutral-700 px-3 py-1 text-sm text-neutral-300 transition-colors hover:border-teal-600 hover:bg-teal-950/50 hover:text-neutral-100 disabled:cursor-default disabled:border-neutral-800 disabled:text-neutral-600 disabled:hover:bg-transparent"
          disabled={exists}
          aria-label={exists ? `${topic} already added` : `Select ${topic}`}
          onclick={() => selectSuggestedTopic(topic)}
        >
          {topic}
        </button>
      {/each}
    </div>
  </section>

  <div class="flex flex-col gap-2">
    {#each $topics as topic (topic.id)}
      <TopicListItem {topic} />
    {/each}
  </div>
</div>
