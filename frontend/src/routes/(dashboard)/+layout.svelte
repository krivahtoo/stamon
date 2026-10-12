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

  onMount(async () => {
    if ($user) {
      const { ws } = await import('$lib/store/ws');
      // Initialize websocket connection and initial states
      if (get(ws.status) === 'CLOSED') {
        ws.open();
      }
      //
      cfetch('/services').then(async (res) => {
        if (res.ok) {
          const data = await res.json();
          // console.log(data);
          services.set(data.services);
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

      <main
        class="flex max-h-[calc(100vh-60px)] flex-1 flex-col gap-4 overflow-y-auto overscroll-y-contain p-4 lg:gap-6 lg:p-6"
        style="will-change: transform; view-transition-name: page;"
      >
        <slot></slot>
      </main>
    </div>
  </div>
{/if}
