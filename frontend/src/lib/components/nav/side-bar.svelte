<script>
  import { afterNavigate } from '$app/navigation';
  import { page } from '$app/stores';
  import { cubicInOut } from 'svelte/easing';
  import { crossfade } from 'svelte/transition';
  import { prefersReducedMotion } from 'svelte/motion';

  import Activity from '@lucide/svelte/icons/activity';
  import * as Sidebar from '$lib/components/ui/sidebar/index.js';
  import { navGroups, visibleGroups, isActive } from '$lib/data/nav.js';
  import instance from '$lib/store/instance.js';
  import stats from '$lib/store/stats.js';
  import user from '$lib/store/user.js';
  import { cn } from '$lib/utils.js';

  const sidebar = Sidebar.useSidebar();

  let activeUrl = $derived($page?.url?.pathname);
  let groups = $derived(visibleGroups(navGroups, $user?.role));

  // The active pill slides between items, across groups too, since every
  // item shares this one pair.
  const [send, receive] = crossfade({
    duration: () => (prefersReducedMotion.current ? 0 : 250),
    easing: cubicInOut
  });

  // Close the mobile drawer once a link has been followed.
  afterNavigate(() => sidebar.setOpenMobile(false));
</script>

<Sidebar.Root collapsible="icon" style="view-transition-name: side-bar;">
  <Sidebar.Header class="px-3 pt-5 pb-1 group-data-[collapsible=icon]:px-2">
    <a
      href="/"
      class="flex h-8 items-center gap-2.5 rounded-lg px-2.5 font-bold tracking-tight group-data-[collapsible=icon]:px-2"
    >
      <Activity class="size-5 shrink-0 text-primary" />
      <span class="group-data-[collapsible=icon]:hidden">Stamon</span>
    </a>
  </Sidebar.Header>

  <Sidebar.Content class="gap-0 px-1">
    {#each groups as group (group.label)}
      <Sidebar.Group class="py-0">
        <Sidebar.GroupLabel
          class="mt-2 px-2.5 text-xs font-bold tracking-[0.08em] text-muted-foreground uppercase"
        >
          {group.label}
        </Sidebar.GroupLabel>
        <Sidebar.Menu class="gap-0.5">
          {#each group.items as item (item.path)}
            {@const active = isActive(item.path, activeUrl)}
            <Sidebar.MenuItem>
              <Sidebar.MenuButton
                isActive={active}
                tooltipContent={item.name}
                class="h-[34px] gap-2.5 rounded-lg px-2.5 text-sm text-muted-foreground data-active:bg-transparent data-active:font-semibold data-active:text-foreground"
              >
                {#snippet child({ props })}
                  <a
                    href={item.path}
                    aria-current={active ? 'page' : undefined}
                    data-sveltekit-noscroll
                    {...props}
                  >
                    {#if active}
                      <div
                        class="absolute inset-0 rounded-lg bg-sidebar-accent"
                        in:send={{ key: 'active-sidebar-item' }}
                        out:receive={{ key: 'active-sidebar-item' }}
                      ></div>
                    {/if}
                    <item.icon class={cn('relative', active && 'text-primary')} />
                    <span class="relative">{item.name}</span>
                  </a>
                {/snippet}
              </Sidebar.MenuButton>
              {#if item.path === '/services/' && $stats.active}
                <Sidebar.MenuBadge class="top-[7px] rounded-full bg-sidebar-accent">
                  {$stats.active}
                </Sidebar.MenuBadge>
              {/if}
            </Sidebar.MenuItem>
          {/each}
        </Sidebar.Menu>
      </Sidebar.Group>
    {/each}
  </Sidebar.Content>

  {#if $instance?.version}
    <Sidebar.Footer class="px-5 pb-5 group-data-[collapsible=icon]:hidden">
      <span class="font-mono text-xs text-muted-foreground">v{$instance.version.replace(/^v/, '')}</span>
    </Sidebar.Footer>
  {/if}
</Sidebar.Root>
