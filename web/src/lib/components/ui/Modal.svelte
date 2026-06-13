<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    open: boolean;
    title?: string;
    onClose: () => void;
    children: Snippet;
  }

  let { open, title, onClose, children }: Props = $props();

  function onKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') onClose();
  }
</script>

<svelte:window onkeydown={open ? onKeydown : undefined} />

{#if open}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/60"
    role="button"
    tabindex="-1"
    aria-label="Close dialog"
    onclick={onClose}
    onkeydown={onKeydown}
  >
    <div
      class="min-w-80 rounded-lg border border-border bg-surface p-4 shadow-xl"
      role="dialog"
      aria-modal="true"
      tabindex="-1"
      onclick={(e) => e.stopPropagation()}
      onkeydown={(e) => e.stopPropagation()}
    >
      {#if title}
        <h2 class="mb-3 text-base font-semibold text-text">{title}</h2>
      {/if}
      {@render children()}
    </div>
  </div>
{/if}
