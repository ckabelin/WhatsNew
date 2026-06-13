<script lang="ts">
  import { Trash2 } from 'lucide-svelte';
  import type { Topic } from '$lib/types';
  import Toggle from '$lib/components/ui/Toggle.svelte';
  import { setTopicNotifications, removeTopic } from '$lib/stores/topics';

  interface Props {
    topic: Topic;
  }

  let { topic }: Props = $props();
</script>

<div class="flex items-center gap-3 rounded-md border border-border bg-surface px-3 py-2">
  <span class="flex-1 truncate text-sm text-text">{topic.name}</span>
  <Toggle
    checked={topic.notifications_enabled}
    label={`Notifications for ${topic.name}`}
    onchange={(enabled) => setTopicNotifications(topic.id, enabled)}
  />
  <button
    class="text-text-muted hover:text-danger"
    aria-label={`Delete ${topic.name}`}
    onclick={() => removeTopic(topic.id)}
  >
    <Trash2 size={16} />
  </button>
</div>
