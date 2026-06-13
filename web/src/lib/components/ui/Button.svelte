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
    primary: 'bg-accent text-accent-fg hover:bg-accent-hover disabled:opacity-50',
    secondary: 'bg-surface-hover text-text hover:bg-active disabled:opacity-50',
    ghost: 'bg-transparent text-text-muted hover:bg-surface-hover disabled:text-text-subtle',
    danger: 'bg-danger text-white hover:bg-danger-hover disabled:opacity-50'
  };
</script>

<button
  {...rest}
  class="inline-flex items-center justify-center gap-2 rounded-md px-3 py-1.5 text-sm font-medium
    transition-colors disabled:cursor-not-allowed {variantClasses[variant]} {className}"
>
  {@render children()}
</button>
