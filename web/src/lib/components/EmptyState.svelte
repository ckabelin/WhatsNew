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
  <h2 class="text-lg font-semibold text-neutral-100">{title}</h2>
  {#if description}
    <p class="max-w-sm text-sm text-neutral-400">{description}</p>
  {/if}
  {#if action}
    <div class="mt-4">
      {@render action()}
    </div>
  {/if}
</div>
