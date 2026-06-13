<script lang="ts">
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { Minus, Square, X } from 'lucide-svelte';

  const appWindow = getCurrentWindow();
</script>

<div
  data-tauri-drag-region
  class="flex h-9 select-none items-center justify-between bg-neutral-950 pl-3"
>
  <span class="text-sm font-medium text-neutral-300">WhatsNew</span>
  <div class="flex h-full">
    <button
      class="inline-flex h-full w-11 items-center justify-center text-neutral-400 hover:bg-neutral-800 hover:text-neutral-100"
      onclick={() => appWindow.hide()}
      aria-label="Minimize to tray"
    >
      <Minus size={16} />
    </button>
    <button
      class="inline-flex h-full w-11 items-center justify-center text-neutral-400 hover:bg-neutral-800 hover:text-neutral-100"
      onclick={() => appWindow.toggleMaximize()}
      aria-label="Maximize"
    >
      <Square size={14} />
    </button>
    <button
      class="inline-flex h-full w-11 items-center justify-center text-neutral-400 hover:bg-red-600 hover:text-white"
      onclick={() => appWindow.close()}
      aria-label="Close"
    >
      <X size={16} />
    </button>
  </div>
</div>
