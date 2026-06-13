<script lang="ts">
  import type { HTMLInputAttributes } from 'svelte/elements';

  interface Props extends Omit<HTMLInputAttributes, 'value'> {
    value?: string | number;
    label?: string;
  }

  let { value = $bindable(''), label, id, class: className = '', ...rest }: Props = $props();

  const generatedId = `input-${Math.random().toString(36).slice(2)}`;
  const inputId = $derived(id ?? generatedId);
</script>

<div class="flex flex-col gap-1">
  {#if label}
    <label for={inputId} class="text-sm font-medium text-neutral-300">{label}</label>
  {/if}
  <input
    id={inputId}
    bind:value
    {...rest}
    class="rounded-md border border-neutral-700 bg-neutral-900 px-3 py-1.5 text-sm text-neutral-100
      placeholder:text-neutral-500 focus:border-blue-500 focus:outline-none {className}"
  />
</div>
