<script>
  import Header from '$lib/components/nav/header.svelte';
  import SideBar from '$lib/components/nav/side-bar.svelte';
  import { onMount } from 'svelte';
  import { fly } from 'svelte/transition';
  import user from '$lib/store/user.js';
  import { connect } from '$lib/store/logs';
  import Home from 'lucide-svelte/icons/home';
  import Layers3 from 'lucide-svelte/icons/layers-3';
  import Users from 'lucide-svelte/icons/users';
  import Monitor from 'lucide-svelte/icons/monitor';
  import MessageSquareText from 'lucide-svelte/icons/message-square-text';

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

  onMount(() => {
    if ($user) {
      // Initialize websocket connection and initial states
      connect();
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
