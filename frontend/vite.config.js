import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vitest/config';

export default defineConfig({
  plugins: [sveltekit()],
  clearScreen: false,
  server: {
    // Same origin as the backend, so the sign-in cookie reaches /ws like in production
    proxy: {
      '/api': 'http://localhost:3000',
      '/ws': { target: 'ws://localhost:3000', ws: true }
    }
  },
  test: {
    include: ['src/**/*.{test,spec}.{js,ts}']
  }
});
