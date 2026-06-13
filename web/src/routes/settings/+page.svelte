<script lang="ts">
  import { onMount } from 'svelte';
  import Input from '$lib/components/ui/Input.svelte';
  import Toggle from '$lib/components/ui/Toggle.svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import { settings, loadSettings, saveSettings } from '$lib/stores/settings';
  import { themeName, themeMode, type ThemeName, type ThemeMode } from '$lib/stores/theme';
  import { autoGroupTopics } from '$lib/stores/viewPreferences';
  import type { Settings } from '$lib/types';

  let form: Settings | null = $state(null);
  let saving = $state(false);

  const themeOptions: { value: ThemeName; label: string }[] = [
    { value: 'vscode', label: 'VS Code' },
    { value: 'outlook', label: 'Outlook' },
    { value: 'console', label: 'Console' }
  ];

  const modeOptions: { value: ThemeMode; label: string }[] = [
    { value: 'dark', label: 'Dark' },
    { value: 'light', label: 'Light' }
  ];

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
  <h1 class="text-lg font-semibold text-text">Settings</h1>

  <section class="flex flex-col gap-2">
    <h2 class="text-sm font-medium text-text-muted">Appearance</h2>
    <div class="flex items-center gap-1 rounded-md border border-border bg-surface p-1">
      {#each themeOptions as option (option.value)}
        <button
          type="button"
          aria-pressed={$themeName === option.value}
          onclick={() => themeName.set(option.value)}
          class="flex-1 rounded-md px-3 py-1.5 text-sm transition-colors
            {$themeName === option.value
            ? 'bg-accent text-accent-fg'
            : 'text-text-muted hover:bg-surface-hover hover:text-text'}"
        >
          {option.label}
        </button>
      {/each}
    </div>
    <div class="flex items-center gap-1 rounded-md border border-border bg-surface p-1">
      {#each modeOptions as option (option.value)}
        <button
          type="button"
          aria-pressed={$themeMode === option.value}
          onclick={() => themeMode.set(option.value)}
          class="flex-1 rounded-md px-3 py-1.5 text-sm transition-colors
            {$themeMode === option.value
            ? 'bg-accent text-accent-fg'
            : 'text-text-muted hover:bg-surface-hover hover:text-text'}"
        >
          {option.label}
        </button>
      {/each}
    </div>
  </section>

  <section class="flex flex-col gap-2">
    <h2 class="text-sm font-medium text-text-muted">Sidebar</h2>
    <div class="flex items-center justify-between">
      <span class="text-sm font-medium text-text-muted">Auto-group topics by category</span>
      <Toggle bind:checked={$autoGroupTopics} label="Auto-group topics by category" />
    </div>
  </section>

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
      <span class="text-sm font-medium text-text-muted">Notifications enabled</span>
      <Toggle bind:checked={form.notifications_enabled} label="Notifications enabled" />
    </div>

    <Button onclick={onSave} disabled={saving}>{saving ? 'Saving…' : 'Save'}</Button>
  {:else}
    <p class="text-sm text-text-muted">Loading settings…</p>
  {/if}
</div>
