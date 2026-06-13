<script lang="ts">
  import type { Snippet } from 'svelte';
  import type { HTMLButtonAttributes } from 'svelte/elements';

  type Variant = 'primary' | 'secondary' | 'ghost' | 'danger';

  interface Props extends HTMLButtonAttributes {
    variant?: Variant;
    children: Snippet;
  }

  let { variant = 'primary', class: className = '', children, ...rest }: Props = $props();

  const variantClasses: Record<Variant, string> = {
    primary: 'bg-blue-600 text-white hover:bg-blue-500 disabled:bg-blue-600/50',
    secondary: 'bg-neutral-800 text-neutral-100 hover:bg-neutral-700 disabled:bg-neutral-800/50',
    ghost: 'bg-transparent text-neutral-300 hover:bg-neutral-800 disabled:text-neutral-500',
    danger: 'bg-red-600 text-white hover:bg-red-500 disabled:bg-red-600/50'
  };
</script>

<button
  {...rest}
  class="inline-flex items-center justify-center gap-2 rounded-md px-3 py-1.5 text-sm font-medium
    transition-colors disabled:cursor-not-allowed {variantClasses[variant]} {className}"
>
  {@render children()}
</button>
