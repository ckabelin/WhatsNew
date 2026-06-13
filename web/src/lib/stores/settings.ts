import { writable } from 'svelte/store';
import type { Settings } from '$lib/types';
import * as settingsApi from '$lib/api/settings';

export const settings = writable<Settings | null>(null);

export async function loadSettings() {
  settings.set(await settingsApi.getSettings());
}

export async function saveSettings(updated: Settings) {
  settings.set(await settingsApi.updateSettings(updated));
}
