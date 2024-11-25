<script>
  import { ModeWatcher } from 'mode-watcher';
  import { Toaster } from '$lib/components/ui/sonner/index.js';
  import { onMount } from 'svelte';
  import user from '$lib/store/user.js';
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { onNavigate } from '$app/navigation';

  import '../app.pcss';

  onNavigate((navigation) => {
    if (!document.startViewTransition) return;

    return new Promise((resolve) => {
      document.startViewTransition(async () => {
        resolve();
        await navigation.complete;
      });
    });
  });

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

<style>
  @keyframes fade-in {
    from {
      opacity: 0;
    }
  }

  @keyframes fade-out {
    to {
      opacity: 0;
    }
  }

  @keyframes slide-from-top {
    from {
      transform: translateY(30px);
    }
  }

  @keyframes slide-to-top {
    to {
      transform: translateY(-30px);
    }
  }

  :root::view-transition-old(page) {
    animation:
      90ms cubic-bezier(0.4, 0, 1, 1) both fade-out,
      300ms cubic-bezier(0.4, 0, 0.2, 1) both slide-to-top;
  }

  :root::view-transition-new(page) {
    animation:
      210ms cubic-bezier(0, 0, 0.2, 1) 90ms both fade-in,
      300ms cubic-bezier(0.4, 0, 0.2, 1) both slide-from-top;
  }
</style>
