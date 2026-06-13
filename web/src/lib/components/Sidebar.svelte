<script lang="ts">
  import { resolve } from '$app/paths';
  import { page } from '$app/stores';
  import { GripVertical, ListTree, MoveDown, MoveUp, Newspaper, Settings } from 'lucide-svelte';
  import { get } from 'svelte/store';
  import { reorderTopics, topics } from '$lib/stores/topics';

  let draggedTopicId = $state<number | null>(null);
  let dropTargetTopicId = $state<number | null>(null);
  let savingOrder = $state(false);

  async function moveTopic(topicId: number, direction: -1 | 1) {
    const list = get(topics);
    const index = list.findIndex((topic) => topic.id === topicId);
    const targetIndex = index + direction;
    if (index < 0 || targetIndex < 0 || targetIndex >= list.length) return;

    const ordered = [...list];
    [ordered[index], ordered[targetIndex]] = [ordered[targetIndex], ordered[index]];
    await saveOrder(ordered.map((topic) => topic.id));
  }

  function onDragStart(event: DragEvent, topicId: number) {
    draggedTopicId = topicId;
    event.dataTransfer?.setData('text/plain', String(topicId));
    if (event.dataTransfer) {
      event.dataTransfer.effectAllowed = 'move';
    }
  }

  function onDragOver(event: DragEvent, topicId: number) {
    if (draggedTopicId === null || draggedTopicId === topicId) return;
    event.preventDefault();
    dropTargetTopicId = topicId;
    if (event.dataTransfer) {
      event.dataTransfer.dropEffect = 'move';
    }
  }

  async function onDrop(event: DragEvent, targetTopicId: number) {
    event.preventDefault();
    const sourceTopicId = Number(event.dataTransfer?.getData('text/plain') || draggedTopicId);
    draggedTopicId = null;
    dropTargetTopicId = null;

    if (!sourceTopicId || sourceTopicId === targetTopicId) return;

    const list = get(topics);
    const sourceIndex = list.findIndex((topic) => topic.id === sourceTopicId);
    const targetIndex = list.findIndex((topic) => topic.id === targetTopicId);
    if (sourceIndex < 0 || targetIndex < 0) return;

    const ordered = [...list];
    const [moved] = ordered.splice(sourceIndex, 1);
    ordered.splice(targetIndex, 0, moved);
    await saveOrder(ordered.map((topic) => topic.id));
  }

  function onDragEnd() {
    draggedTopicId = null;
    dropTargetTopicId = null;
  }

  async function saveOrder(topicIds: number[]) {
    savingOrder = true;
    try {
      await reorderTopics(topicIds);
    } finally {
      savingOrder = false;
    }
  }
</script>

<aside
  class="flex w-56 flex-col gap-1 overflow-y-auto border-r border-neutral-800 bg-neutral-950 p-2"
>
  <div class="flex flex-col gap-1" role="list" aria-label="Topics">
    {#each $topics as topic, index (topic.id)}
      <div
        role="listitem"
        class="group flex items-center rounded-md transition-colors
          {dropTargetTopicId === topic.id ? 'bg-teal-950/70 ring-1 ring-teal-700' : ''}
          {$page.params.id === String(topic.id)
          ? 'bg-neutral-800 text-neutral-100'
          : 'text-neutral-400 hover:bg-neutral-800 hover:text-neutral-100'}"
        draggable={!savingOrder}
        ondragstart={(event) => onDragStart(event, topic.id)}
        ondragover={(event) => onDragOver(event, topic.id)}
        ondrop={(event) => onDrop(event, topic.id)}
        ondragend={onDragEnd}
      >
        <GripVertical size={14} class="ml-1 shrink-0 cursor-grab text-neutral-600" />
        <a
          href={resolve('/topic/[id]', { id: String(topic.id) })}
          class="flex min-w-0 flex-1 items-center gap-2 px-1 py-1.5 text-sm"
        >
          <Newspaper size={16} class="shrink-0" />
          <span class="truncate">{topic.name}</span>
        </a>
        <div class="mr-1 hidden items-center gap-0.5 group-hover:flex group-focus-within:flex">
          <button
            type="button"
            class="rounded p-1 text-neutral-500 hover:bg-neutral-700 hover:text-neutral-100 disabled:opacity-30"
            disabled={index === 0 || savingOrder}
            aria-label="Move topic up"
            onclick={() => moveTopic(topic.id, -1)}
          >
            <MoveUp size={13} />
          </button>
          <button
            type="button"
            class="rounded p-1 text-neutral-500 hover:bg-neutral-700 hover:text-neutral-100 disabled:opacity-30"
            disabled={index === $topics.length - 1 || savingOrder}
            aria-label="Move topic down"
            onclick={() => moveTopic(topic.id, 1)}
          >
            <MoveDown size={13} />
          </button>
        </div>
      </div>
    {/each}
  </div>

  <div class="mt-auto flex flex-col gap-1 border-t border-neutral-800 pt-2">
    <a
      href={resolve('/topics')}
      class="flex items-center gap-2 rounded-md px-2 py-1.5 text-sm transition-colors
        {$page.url.pathname === '/topics'
        ? 'bg-neutral-800 text-neutral-100'
        : 'text-neutral-400 hover:bg-neutral-800 hover:text-neutral-100'}"
    >
      <ListTree size={16} />
      <span>Topics</span>
    </a>
    <a
      href={resolve('/settings')}
      class="flex items-center gap-2 rounded-md px-2 py-1.5 text-sm transition-colors
        {$page.url.pathname === '/settings'
        ? 'bg-neutral-800 text-neutral-100'
        : 'text-neutral-400 hover:bg-neutral-800 hover:text-neutral-100'}"
    >
      <Settings size={16} />
      <span>Settings</span>
    </a>
  </div>
</aside>
