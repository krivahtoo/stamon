import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';
import adapter from '@sveltejs/adapter-static';

/** @type {import('@sveltejs/kit').Config} */
const config = {
  kit: {
    // See https://kit.svelte.dev/docs/adapters for more information about adapters.
    adapter: adapter({
      pages: 'build',
      strict: true,
      fallback: '404.html'
    }),

    alias: {
      '@/*': './src/lib/*'
    }
  },

  preprocess: [vitePreprocess({})]
};

export default config;
