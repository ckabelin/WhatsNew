<script lang="ts">
  interface Props {
    checked?: boolean;
    disabled?: boolean;
    label?: string;
    onchange?: (checked: boolean) => void;
  }

  let { checked = $bindable(false), disabled = false, label, onchange }: Props = $props();

  function toggle() {
    if (disabled) return;
    checked = !checked;
    onchange?.(checked);
  }
</script>

<button
  type="button"
  role="switch"
  aria-checked={checked}
  aria-label={label}
  {disabled}
  onclick={toggle}
  class="relative inline-flex h-5 w-9 shrink-0 items-center rounded-full transition-colors
    disabled:cursor-not-allowed disabled:opacity-50 {checked ? 'bg-blue-600' : 'bg-neutral-700'}"
>
  <span
    class="inline-block h-4 w-4 transform rounded-full bg-white transition-transform
      {checked ? 'translate-x-[18px]' : 'translate-x-0.5'}"
  ></span>
</button>
