<script>
  import Header from '$lib/components/nav/header.svelte';
  import SideBar from '$lib/components/nav/side-bar.svelte';
  import { onMount } from 'svelte';
  import { fly } from 'svelte/transition';
  import user from '$lib/store/user.js';
  import Home from 'lucide-svelte/icons/home';
  import Layers3 from 'lucide-svelte/icons/layers-3';
  import Users from 'lucide-svelte/icons/users';
  import Monitor from 'lucide-svelte/icons/monitor';
  import MessageSquareText from 'lucide-svelte/icons/message-square-text';
  import { get } from 'svelte/store';
  import { cfetch } from '$lib/utils.js';
  import services from '$lib/store/services.js';
  import stats from '$lib/store/stats.js';

  let navItems = [
    {
      name: 'Dashboard',
      path: '/',
      icon: Home
    },
    {
      name: 'Services',
      // final '/' is important for hightlighting
      // current active navitem
      path: '/services/',
      icon: Layers3
    },
    {
      name: 'Status Pages',
      path: '/status-pages/',
      icon: Monitor
    },
    {
      name: 'Notification Providers',
      path: '/notifications/',
      icon: MessageSquareText
    },
    {
      name: 'Team',
      path: '/users/',
      icon: Users
    }
  ];

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
  <div class="grid min-h-screen w-full md:grid-cols-[220px_1fr] lg:grid-cols-[280px_1fr]">
    <SideBar {navItems} />

    <div class="flex flex-col">
      <Header {navItems} />

      <main class="flex flex-1 flex-col gap-4 p-4 lg:gap-6 lg:p-6">
        <slot></slot>
      </main>
    </div>
  </div>
{/if}