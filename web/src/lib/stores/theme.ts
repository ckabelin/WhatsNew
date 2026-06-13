import { writable } from 'svelte/store';
import { load, type Store } from '@tauri-apps/plugin-store';

export type ThemeName = 'vscode' | 'outlook' | 'console';
export type ThemeMode = 'dark' | 'light';

export const themeName = writable<ThemeName>('vscode');
export const themeMode = writable<ThemeMode>('dark');

const STORE_FILE = 'theme-preferences.json';
let store: Store | undefined;

export async function initTheme() {
  if (store) return;
  store = await load(STORE_FILE);

  const savedTheme = await store.get<ThemeName>('themeName');
  const savedMode = await store.get<ThemeMode>('themeMode');

  if (savedTheme) themeName.set(savedTheme);
  if (savedMode) themeMode.set(savedMode);

  themeName.subscribe((value) => void store?.set('themeName', value));
  themeMode.subscribe((value) => void store?.set('themeMode', value));
}
