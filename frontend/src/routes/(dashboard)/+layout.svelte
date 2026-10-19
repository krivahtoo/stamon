<script>
  import Header from '$lib/components/nav/header.svelte';
  import SideBar from '$lib/components/nav/side-bar.svelte';
  import { onMount } from 'svelte';
  import user from '$lib/store/user.js';
  import { get } from 'svelte/store';
  import { cfetch } from '$lib/utils.js';
  import { navItems } from '$lib/data/nav.js';
  import services from '$lib/store/services.js';
  import stats from '$lib/store/stats.js';
  import demo from '$lib/store/demo.js';
  import FlaskConical from 'lucide-svelte/icons/flask-conical';
  import X from 'lucide-svelte/icons/x';

  let showDemoBanner = true;

  onMount(async () => {
    if ($user) {
      const { ws } = await import('$lib/store/ws');
      // Initialize websocket connection and initial states
      if (get(ws.status) === 'CLOSED') {
        // A failed connection must not stop the initial data from loading
        try {
          ws.open();
        } catch (err) {
          console.error(err);
        }
      }
      //
      cfetch('/services').then(async (res) => {
        if (res.ok) {
          const data = await res.json();
          // console.log(data);
          services.set(data.services);
        }
      });

      cfetch('/check').then(async (res) => {
        if (res.ok) {
          const data = await res.json();
          demo.set(!!data.demo);
        }
      });

      cfetch('/stats').then(async (res) => {
        if (res.ok) {
          const data = await res.json();
          // console.log(data);
          stats.set(data.stats);
        }
      });
    }
  });
</script>

{#if $user}
  <div class="grid h-screen w-full overflow-auto md:grid-cols-[220px_1fr] lg:grid-cols-[280px_1fr]">
    <SideBar {navItems} />

    <div class="flex flex-col">
      <Header {navItems} />

      {#if $demo && showDemoBanner}
        <div
          class="flex items-center gap-2 border-b bg-muted/50 px-4 py-2 text-sm text-muted-foreground lg:px-6"
        >
          <FlaskConical class="h-4 w-4 shrink-0" />
          <p class="flex-1">
            This is the public demo: data resets every hour and alerts aren't sent.
            <a href="https://stamon.org/" class="font-medium underline underline-offset-4">
              Install Stamon
            </a>
            to monitor your own services.
          </p>
          <button
            type="button"
            class="rounded p-1 hover:bg-muted"
            aria-label="Dismiss"
            on:click={() => (showDemoBanner = false)}
          >
            <X class="h-4 w-4" />
          </button>
        </div>
      {/if}

      <main
        class="flex max-h-[calc(100vh-60px)] flex-1 flex-col gap-4 overflow-y-auto overscroll-y-contain p-4 lg:gap-6 lg:p-6"
        style="will-change: transform; view-transition-name: page;"
      >
        <slot></slot>
      </main>
    </div>
  </div>
{/if}
