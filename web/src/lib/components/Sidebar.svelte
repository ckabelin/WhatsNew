<script lang="ts">
  import { resolve } from '$app/paths';
  import { page } from '$app/stores';
  import {
    Bookmark,
    ChevronDown,
    ChevronRight,
    GripVertical,
    ListTree,
    MoveDown,
    MoveUp,
    Search,
    Settings,
    X
  } from 'lucide-svelte';
  import { get } from 'svelte/store';
  import { reorderTopics, topics } from '$lib/stores/topics';
  import { getTopicIcon } from '$lib/topicIcon';
  import { getTopicCategory } from '$lib/topicCategory';
  import {
    autoGroupTopics,
    expandedTopicGroup,
    sidebarWidth,
    topicGroupOrder
  } from '$lib/stores/viewPreferences';
  import type { Topic } from '$lib/types';

  const MIN_SIDEBAR_WIDTH = 180;
  const MAX_SIDEBAR_WIDTH = 480;

  let draggedTopicId = $state<number | null>(null);
  let dropTargetTopicId = $state<number | null>(null);
  let draggedGroupLabel = $state<string | null>(null);
  let dropTargetGroupLabel = $state<string | null>(null);
  let savingOrder = $state(false);
  let filterQuery = $state('');
  let resizing = $state(false);

  const filteredTopics = $derived(
    filterQuery.trim()
      ? $topics.filter((topic) =>
          topic.name.toLocaleLowerCase().includes(filterQuery.trim().toLocaleLowerCase())
        )
      : $topics
  );

  /**
   * Group display order: any user-customized order (from dragging group
   * headers) takes precedence, falling back to each group's first-appearance
   * position in the topic list - so newly created topics land their group
   * near where the user already organized similar topics, rather than at an
   * arbitrary fixed position.
   */
  const groupOrder = $derived.by(() => {
    const presentLabels = new Set(
      filteredTopics.map((topic) => getTopicCategory(topic.name).label)
    );
    const appearanceOrder = [
      ...new Set($topics.map((topic) => getTopicCategory(topic.name).label))
    ];
    const custom = $topicGroupOrder.filter((label) => presentLabels.has(label));
    const remaining = appearanceOrder.filter(
      (label) => presentLabels.has(label) && !custom.includes(label)
    );
    return [...custom, ...remaining];
  });

  const groupedTopics = $derived.by(() => {
    const groups: Record<string, Topic[]> = {};
    for (const topic of filteredTopics) {
      const label = getTopicCategory(topic.name).label;
      (groups[label] ??= []).push(topic);
    }
    return groupOrder.map((label) => ({ label, topics: groups[label] ?? [] }));
  });

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

  /**
   * Accordion behavior: at most one group is expanded at a time. Toggling the
   * currently expanded group collapses it, leaving all groups collapsed.
   */
  function toggleGroup(label: string) {
    expandedTopicGroup.update((expanded) => (expanded === label ? null : label));
  }

  function onGroupDragStart(event: DragEvent, label: string) {
    draggedGroupLabel = label;
    event.dataTransfer?.setData('text/plain', label);
    if (event.dataTransfer) {
      event.dataTransfer.effectAllowed = 'move';
    }
  }

  function onGroupDragOver(event: DragEvent, label: string) {
    if (draggedGroupLabel === null || draggedGroupLabel === label) return;
    event.preventDefault();
    dropTargetGroupLabel = label;
    if (event.dataTransfer) {
      event.dataTransfer.dropEffect = 'move';
    }
  }

  function onGroupDrop(event: DragEvent, targetLabel: string) {
    event.preventDefault();
    const sourceLabel = event.dataTransfer?.getData('text/plain') || draggedGroupLabel;
    draggedGroupLabel = null;
    dropTargetGroupLabel = null;

    if (!sourceLabel || sourceLabel === targetLabel) return;

    const order = [...groupOrder];
    const sourceIndex = order.indexOf(sourceLabel);
    const targetIndex = order.indexOf(targetLabel);
    if (sourceIndex < 0 || targetIndex < 0) return;

    const [moved] = order.splice(sourceIndex, 1);
    order.splice(targetIndex, 0, moved);
    topicGroupOrder.set(order);
  }

  function onGroupDragEnd() {
    draggedGroupLabel = null;
    dropTargetGroupLabel = null;
  }

  function onResizeStart(event: PointerEvent) {
    event.preventDefault();
    resizing = true;

    const startX = event.clientX;
    const startWidth = get(sidebarWidth);

    function onPointerMove(moveEvent: PointerEvent) {
      const next = startWidth + (moveEvent.clientX - startX);
      sidebarWidth.set(Math.min(MAX_SIDEBAR_WIDTH, Math.max(MIN_SIDEBAR_WIDTH, next)));
    }

    function onPointerUp() {
      resizing = false;
      window.removeEventListener('pointermove', onPointerMove);
      window.removeEventListener('pointerup', onPointerUp);
    }

    window.addEventListener('pointermove', onPointerMove);
    window.addEventListener('pointerup', onPointerUp);
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
  class="relative flex flex-col gap-1 overflow-hidden border-r border-border bg-bg p-2"
  style="width: {$sidebarWidth}px; flex: 0 0 {$sidebarWidth}px;"
>
  {#if $topics.length > 5}
    <div class="relative px-1 pb-1">
      <Search
        size={13}
        class="pointer-events-none absolute left-3 top-1/2 -translate-y-1/2 text-text-subtle"
      />
      <input
        type="text"
        bind:value={filterQuery}
        placeholder="Filter topics"
        aria-label="Filter topics"
        class="w-full rounded-md border border-border-strong bg-surface py-1 pl-7 pr-6 text-xs text-text
          placeholder:text-text-subtle focus:border-accent focus:outline-none"
      />
      {#if filterQuery}
        <button
          type="button"
          aria-label="Clear filter"
          class="absolute right-2 top-1/2 -translate-y-1/2 text-text-subtle hover:text-text"
          onclick={() => (filterQuery = '')}
        >
          <X size={13} />
        </button>
      {/if}
    </div>
  {/if}
  <div class="flex min-h-0 flex-1 flex-col gap-1 overflow-y-auto" role="list" aria-label="Topics">
    {#if $autoGroupTopics && !filterQuery.trim()}
      {#each groupedTopics as group (group.label)}
        {@const collapsed = $expandedTopicGroup !== group.label}
        <div
          role="group"
          aria-label="{group.label} topics"
          class="group flex items-center gap-0.5 rounded-md px-1 pb-1 pt-2 transition-colors
            first:pt-0
            {dropTargetGroupLabel === group.label ? 'bg-highlight ring-1 ring-highlight-ring' : ''}"
          draggable={true}
          ondragstart={(event) => onGroupDragStart(event, group.label)}
          ondragover={(event) => onGroupDragOver(event, group.label)}
          ondrop={(event) => onGroupDrop(event, group.label)}
          ondragend={onGroupDragEnd}
        >
          <GripVertical
            size={12}
            class="shrink-0 cursor-grab text-text-subtle opacity-0 group-hover:opacity-100"
          />
          <button
            type="button"
            class="flex flex-1 items-center gap-1 text-[10px] font-semibold uppercase
              tracking-wide text-text-subtle hover:text-text"
            aria-expanded={!collapsed}
            onclick={() => toggleGroup(group.label)}
          >
            {#if collapsed}
              <ChevronRight size={11} class="shrink-0" />
            {:else}
              <ChevronDown size={11} class="shrink-0" />
            {/if}
            <span>{group.label}</span>
          </button>
        </div>
        {#if !collapsed}
          {#each group.topics as topic (topic.id)}
            {@render topicRow(topic)}
          {/each}
        {/if}
      {/each}
    {:else}
      {#each filteredTopics as topic (topic.id)}
        {@render topicRow(topic)}
      {/each}
    {/if}
    {#if $topics.length > 0 && filteredTopics.length === 0}
      <p class="px-2 py-1.5 text-xs text-text-subtle">No topics match "{filterQuery}".</p>
    {/if}
  </div>

  <div class="mt-auto flex flex-col gap-1 border-t border-border pt-2">
    <a
      href={resolve('/search')}
      class="flex items-center gap-2 rounded-md px-2 py-1.5 text-sm transition-colors
        {$page.url.pathname === '/search' || $page.url.pathname === '/search/read'
        ? 'bg-active text-text'
        : 'text-text-muted hover:bg-surface-hover hover:text-text'}"
    >
      <Search size={16} />
      <span>Search</span>
    </a>
    <a
      href={resolve('/favorites')}
      class="flex items-center gap-2 rounded-md px-2 py-1.5 text-sm transition-colors
        {$page.url.pathname === '/favorites'
        ? 'bg-active text-text'
        : 'text-text-muted hover:bg-surface-hover hover:text-text'}"
    >
      <Bookmark size={16} />
      <span>Bookmarks</span>
    </a>
    <a
      href={resolve('/topics')}
      class="flex items-center gap-2 rounded-md px-2 py-1.5 text-sm transition-colors
        {$page.url.pathname === '/topics'
        ? 'bg-active text-text'
        : 'text-text-muted hover:bg-surface-hover hover:text-text'}"
    >
      <ListTree size={16} />
      <span>Topics</span>
    </a>
    <a
      href={resolve('/settings')}
      class="flex items-center gap-2 rounded-md px-2 py-1.5 text-sm transition-colors
        {$page.url.pathname === '/settings'
        ? 'bg-active text-text'
        : 'text-text-muted hover:bg-surface-hover hover:text-text'}"
    >
      <Settings size={16} />
      <span>Settings</span>
    </a>
  </div>

  <!-- eslint-disable-next-line svelte/no-static-element-interactions -->
  <div
    role="separator"
    aria-orientation="vertical"
    aria-label="Resize sidebar"
    class="absolute right-0 top-0 h-full w-1 cursor-col-resize transition-colors hover:bg-accent
      {resizing ? 'bg-accent' : ''}"
    onpointerdown={onResizeStart}
  ></div>
</aside>

{#snippet topicRow(topic: Topic)}
  {@const index = $topics.findIndex((t) => t.id === topic.id)}
  {@const TopicIcon = getTopicIcon(topic.name)}
  <div
    role="listitem"
    class="group flex items-center rounded-md transition-colors
      {dropTargetTopicId === topic.id ? 'bg-highlight ring-1 ring-highlight-ring' : ''}
      {$page.params.id === String(topic.id)
      ? 'bg-active text-text'
      : 'text-text-muted hover:bg-surface-hover hover:text-text'}"
    draggable={!savingOrder}
    ondragstart={(event) => onDragStart(event, topic.id)}
    ondragover={(event) => onDragOver(event, topic.id)}
    ondrop={(event) => onDrop(event, topic.id)}
    ondragend={onDragEnd}
  >
    <GripVertical size={14} class="ml-1 shrink-0 cursor-grab text-text-subtle" />
    <a
      href={resolve('/topic/[id]', { id: String(topic.id) })}
      class="flex min-w-0 flex-1 items-center gap-2 px-1 py-1.5 text-sm"
    >
      <TopicIcon size={16} class="shrink-0" />
      <span class="truncate">{topic.name}</span>
    </a>
    <div class="mr-1 hidden items-center gap-0.5 group-hover:flex group-focus-within:flex">
      <button
        type="button"
        class="rounded p-1 text-text-subtle hover:bg-active hover:text-text disabled:opacity-30"
        disabled={index === 0 || savingOrder}
        aria-label="Move topic up"
        onclick={() => moveTopic(topic.id, -1)}
      >
        <MoveUp size={13} />
      </button>
      <button
        type="button"
        class="rounded p-1 text-text-subtle hover:bg-active hover:text-text disabled:opacity-30"
        disabled={index === $topics.length - 1 || savingOrder}
        aria-label="Move topic down"
        onclick={() => moveTopic(topic.id, 1)}
      >
        <MoveDown size={13} />
      </button>
    </div>
  </div>
{/snippet}
