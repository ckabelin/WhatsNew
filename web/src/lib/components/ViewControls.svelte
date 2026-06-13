<script lang="ts">
  import { LayoutGrid, List, Newspaper, Rows3 } from 'lucide-svelte';
  import { viewMode, sortMode, type ViewMode, type SortMode } from '$lib/stores/viewPreferences';

  const viewOptions: { value: ViewMode; label: string; icon: typeof LayoutGrid }[] = [
    { value: 'grid', label: 'Grid', icon: LayoutGrid },
    { value: 'list', label: 'Compact list', icon: List },
    { value: 'magazine', label: 'Magazine', icon: Newspaper },
    { value: 'headlines', label: 'Headlines', icon: Rows3 }
  ];

  const sortOptions: { value: SortMode; label: string }[] = [
    { value: 'newest', label: 'Newest first' },
    { value: 'oldest', label: 'Oldest first' },
    { value: 'relevancy', label: 'Most relevant' }
  ];
</script>

<div class="flex flex-wrap items-center gap-3">
  <div class="flex items-center gap-1 rounded-md border border-border bg-surface p-1">
    {#each viewOptions as option (option.value)}
      <button
        type="button"
        title={option.label}
        aria-label={option.label}
        aria-pressed={$viewMode === option.value}
        onclick={() => viewMode.set(option.value)}
        class="inline-flex items-center justify-center rounded-md p-1.5 transition-colors
          {$viewMode === option.value
          ? 'bg-accent text-accent-fg'
          : 'text-text-muted hover:bg-surface-hover hover:text-text'}"
      >
        <option.icon size={16} />
      </button>
    {/each}
  </div>

  <select
    bind:value={$sortMode}
    aria-label="Sort articles"
    class="rounded-md border border-border bg-surface px-2 py-1.5 text-sm text-text
      focus:outline-none focus:ring-1 focus:ring-accent"
  >
    {#each sortOptions as option (option.value)}
      <option value={option.value}>{option.label}</option>
    {/each}
  </select>
</div>
