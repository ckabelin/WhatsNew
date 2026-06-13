import { invoke } from '@tauri-apps/api/core';
import type { Settings } from '$lib/types';

export const getSettings = () => invoke<Settings>('get_settings');
export const updateSettings = (settings: Settings) =>
  invoke<Settings>('update_settings', { settings });
