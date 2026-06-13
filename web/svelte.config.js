import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: vitePreprocess(),
  kit: {
    // WhatsNew is a Tauri desktop app: the frontend is exported as static
    // files and served from the WebView, so there is no SSR server.
    adapter: adapter({
      fallback: 'index.html'
    })
  }
};

export default config;
