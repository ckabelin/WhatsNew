<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    title: string;
    description?: string;
    action?: Snippet;
  }

  let { title, description, action }: Props = $props();
</script>

<div class="flex h-full flex-col items-center justify-center gap-2 p-8 text-center">
  <h2 class="text-lg font-semibold text-text">{title}</h2>
  {#if description}
    <p class="max-w-md whitespace-pre-line text-sm text-text-muted">{description}</p>
  {/if}
  {#if action}
    <div class="mt-4">
      {@render action()}
    </div>
  {/if}
</div>
