<script>
  import { ModeWatcher } from 'mode-watcher';
  import { Toaster } from '$lib/components/ui/sonner/index.js';
  import { onMount } from 'svelte';
  import user from '$lib/store/user.js';
  import { page } from '$app/stores';
  import { dev } from '$app/environment';
  import { goto } from '$app/navigation';

  import '../app.pcss';

  onMount(() => {
    if (!$user && $page?.url?.pathname != '/login/' && $page?.url?.pathname != '/register/') {
      goto('/login');
    }
  });
</script>

<ModeWatcher />

<Toaster richColors closeButton />

<div class="flex min-h-screen w-full flex-col dark:border-primary/50">
  <slot></slot>
</div>