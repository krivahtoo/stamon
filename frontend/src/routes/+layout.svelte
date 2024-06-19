<script>
  import { page } from '$app/stores';
  import { ModeWatcher } from 'mode-watcher';

  import '../app.pcss';
  import Header from './Header.svelte';
</script>

<ModeWatcher />

<div class="flex min-h-screen w-full flex-col">
  {#if $page.url.pathname !== '/login'}
    <Header></Header>
  {/if}

  <main class="grid flex-1 items-start gap-4 p-4 sm:px-6 sm:py-0 md:gap-8">
    <slot></slot>
  </main>
</div>
