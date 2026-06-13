<script lang="ts">
  import { onMount } from 'svelte';
  import Input from '$lib/components/ui/Input.svelte';
  import Toggle from '$lib/components/ui/Toggle.svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import { settings, loadSettings, saveSettings } from '$lib/stores/settings';
  import type { Settings } from '$lib/types';

  let form: Settings | null = $state(null);
  let saving = $state(false);

  onMount(async () => {
    await loadSettings();
    form = $settings ? { ...$settings } : null;
  });

  async function onSave() {
    if (!form) return;
    saving = true;
    try {
      await saveSettings(form);
    } finally {
      saving = false;
    }
  }
</script>

<div class="flex max-w-md flex-col gap-4 p-4">
  <h1 class="text-lg font-semibold text-neutral-100">Settings</h1>

  {#if form}
    <Input label="Retention (days)" type="number" min="1" bind:value={form.retention_days} />
    <Input
      label="Max articles per topic"
      type="number"
      min="1"
      bind:value={form.max_articles_per_topic}
    />
    <Input label="Max cache size (MB)" type="number" min="1" bind:value={form.max_cache_size_mb} />
    <Input
      label="Refresh interval (minutes)"
      type="number"
      min="1"
      bind:value={form.refresh_interval_minutes}
    />

    <div class="flex items-center justify-between">
      <span class="text-sm font-medium text-neutral-300">Notifications enabled</span>
      <Toggle bind:checked={form.notifications_enabled} label="Notifications enabled" />
    </div>

    <Button onclick={onSave} disabled={saving}>{saving ? 'Saving…' : 'Save'}</Button>
  {:else}
    <p class="text-sm text-neutral-400">Loading settings…</p>
  {/if}
</div>
