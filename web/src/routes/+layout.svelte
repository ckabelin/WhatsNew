<script lang="ts">
  import '../app.css';
  import type { Snippet } from 'svelte';
  import { onMount } from 'svelte';
  import TitleBar from '$lib/components/TitleBar.svelte';
  import Sidebar from '$lib/components/Sidebar.svelte';
  import { loadTopics } from '$lib/stores/topics';

  interface Props {
    children: Snippet;
  }

  let { children }: Props = $props();

  onMount(() => {
    loadTopics();
  });
</script>

<div class="flex h-screen flex-col overflow-hidden">
  <TitleBar />
  <div class="flex flex-1 overflow-hidden">
    <Sidebar />
    <main class="flex-1 overflow-y-auto">
      {@render children()}
    </main>
  </div>
</div>
