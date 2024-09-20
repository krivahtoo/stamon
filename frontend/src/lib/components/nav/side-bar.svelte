<script>
  import { page } from '$app/stores';
  import { cubicInOut } from 'svelte/easing';
  import { crossfade } from 'svelte/transition';

  import { Badge } from '$lib/components/ui/badge/index.js';
  import Activity from 'lucide-svelte/icons/activity';
  import stats from '$lib/store/stats.js';
  import { cn } from '$lib/utils.js';

  import SideBarCard from './side-bar-card.svelte';
  import NotificationPopover from './notification-popover.svelte';

  /**
   * @typedef {Object} NavItem
   * @property {string} name - Name to show in the navbar
   * @property {string} path - navication item path
   * @property {import('svelte').ComponentType} icon - navication item icon
   */

  /** @type {NavItem[]} */
  export let navItems = [];

  // Subscribe to the page path
  $: activeUrl = $page?.url?.pathname;

  const [send, receive] = crossfade({
    duration: 250,
    easing: cubicInOut
  });
</script>

<div class="hidden border-r bg-muted/40 dark:border-primary/50 md:block">
  <div class="flex h-full max-h-screen flex-col gap-2">
    <div class="flex h-14 items-center border-b px-4 dark:border-primary/50 lg:h-[60px] lg:px-6">
      <a href="/" class="flex items-center gap-2 font-semibold">
        <Activity class="h-6 w-6" />
        <span class="">Stamon</span>
      </a>
      <NotificationPopover />
    </div>
    <div class="flex-1">
      <nav class="grid items-start px-2 text-sm font-medium lg:px-4">
        {#each navItems as item, idx (idx)}
          {@const isActive = activeUrl === item.path}
          <a
            href={item.path}
            class={cn(
              !isActive && 'hover:text-primary',
              'relative justify-start hover:bg-transparent'
            )}
            data-sveltekit-noscroll
            data-sveltekit-replacestate
          >
            {#if isActive}
              <div
                class="absolute inset-0 rounded-md bg-background/40"
                in:send={{ key: 'active-sidebar-item' }}
                out:receive={{ key: 'active-sidebar-item' }}
              />
            {/if}
            <div
              class={cn(
                isActive && 'text-primary',
                !isActive && 'text-muted-foreground hover:text-primary',
                'relative flex items-center gap-3 rounded-lg px-3 py-2 transition-all'
              )}
            >
              <svelte:component this={item.icon} class="h-4 w-4" />
              {item.name}
              {#if item.name == 'Services'}
                {#if $stats.active}
                  <Badge
                    class="ml-auto flex h-6 w-6 shrink-0 items-center justify-center rounded-full"
                  >
                    {$stats.active}
                  </Badge>
                {/if}
              {/if}
            </div>
          </a>
        {/each}
      </nav>
    </div>
    <div class="mt-auto p-4">
      <!-- <SideBarCard/> -->
    </div>
  </div>
</div>
