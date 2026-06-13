<script lang="ts">
  import { resolve } from '$app/paths';
  import { page } from '$app/stores';
  import { Newspaper, ListTree, Settings } from 'lucide-svelte';
  import { topics } from '$lib/stores/topics';
</script>

<aside
  class="flex w-56 flex-col gap-1 overflow-y-auto border-r border-neutral-800 bg-neutral-950 p-2"
>
  <div class="flex flex-col gap-1">
    {#each $topics as topic (topic.id)}
      <a
        href={resolve('/topic/[id]', { id: String(topic.id) })}
        class="flex items-center gap-2 rounded-md px-2 py-1.5 text-sm transition-colors
          {$page.params.id === String(topic.id)
          ? 'bg-neutral-800 text-neutral-100'
          : 'text-neutral-400 hover:bg-neutral-800 hover:text-neutral-100'}"
      >
        <Newspaper size={16} />
        <span class="truncate">{topic.name}</span>
      </a>
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
