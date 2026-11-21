<script>
  import Header from '$lib/components/nav/header.svelte';
  import SideBar from '$lib/components/nav/side-bar.svelte';
  import { onMount } from 'svelte';
  import user from '$lib/store/user.js';
  import { get } from 'svelte/store';
  import { cfetch } from '$lib/utils.js';
  import * as Sidebar from '$lib/components/ui/sidebar/index.js';
  import { SIDEBAR_COOKIE_NAME } from '$lib/components/ui/sidebar/constants.js';
  import instance from '$lib/store/instance.js';
  import services from '$lib/store/services.js';
  import stats from '$lib/store/stats.js';
  import demo from '$lib/store/demo.js';
  import FlaskConical from '@lucide/svelte/icons/flask-conical';
  import X from '@lucide/svelte/icons/x';
  /**
   * @typedef {Object} Props
   * @property {import('svelte').Snippet} [children]
   */

  /** @type {Props} */
  let { children } = $props();

  let showDemoBanner = $state(true);
  let sidebarOpen = $state(true);

  onMount(async () => {
    // Pages are prerendered, so the saved sidebar state is only known here.
    sidebarOpen = !document.cookie.includes(`${SIDEBAR_COOKIE_NAME}=false`);

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

      cfetch('/settings').then(async (res) => {
        if (res.ok) {
          const data = await res.json();
          instance.set(data.instance);
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
  <Sidebar.Provider bind:open={sidebarOpen} style="--sidebar-width: 15rem;" class="h-svh">
    <SideBar />

    <Sidebar.Inset class="min-w-0 overflow-y-auto overscroll-y-contain">
      <Header />

      {#if $demo && showDemoBanner}
        <div
          class="flex items-center gap-2 border-b bg-muted/50 px-4 py-2 text-sm text-muted-foreground md:px-8"
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
            onclick={() => (showDemoBanner = false)}
          >
            <X class="h-4 w-4" />
          </button>
        </div>
      {/if}

      <div
        class="flex flex-1 flex-col gap-5 px-4 pt-6 pb-8 md:gap-6 md:p-8"
        style="view-transition-name: page;"
      >
        {@render children?.()}
      </div>
    </Sidebar.Inset>
  </Sidebar.Provider>
{/if}
