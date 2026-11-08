<script>
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { resetMode, setMode } from 'mode-watcher';
  import Activity from '@lucide/svelte/icons/activity';
  import ChevronDown from '@lucide/svelte/icons/chevron-down';
  import Search from '@lucide/svelte/icons/search';
  import Sun from '@lucide/svelte/icons/sun';
  import Moon from '@lucide/svelte/icons/moon';
  import Settings from '@lucide/svelte/icons/settings';
  import LogOut from '@lucide/svelte/icons/log-out';
  import UserRoundCog from '@lucide/svelte/icons/user-round-cog';

  import * as Avatar from '$lib/components/ui/avatar/index.js';
  import * as Breadcrumb from '$lib/components/ui/breadcrumb/index.js';
  import { Button } from '$lib/components/ui/button/index.js';
  import * as Dialog from '$lib/components/ui/dialog/index.js';
  import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
  import * as Sidebar from '$lib/components/ui/sidebar/index.js';
  import services from '$lib/store/services.js';
  import user from '$lib/store/user.js';
  import { cfetch } from '$lib/utils.js';

  import { crumbs } from './breadcrumbs.js';
  import NotificationPopover from './notification-popover.svelte';
  import ServiceSearch from './service-search.svelte';

  const sidebar = Sidebar.useSidebar();

  /** @type {HTMLInputElement | null} */
  let searchInput = $state(null);
  let searchOpen = $state(false);

  let trail = $derived(crumbs($page.url.pathname, $services));
  let initials = $derived(($user?.username ?? '?').slice(0, 2).toUpperCase());

  /** @param {KeyboardEvent} e */
  function onkeydown(e) {
    if (e.key.toLowerCase() === 'k' && (e.metaKey || e.ctrlKey)) {
      e.preventDefault();
      if (sidebar.isMobile) searchOpen = true;
      else searchInput?.focus();
    }
  }

  function logout() {
    cfetch('/logout').finally(() => {
      user.set(null);
      goto('/login');
    });
  }
</script>

<svelte:window {onkeydown} />

<header
  class="sticky top-0 z-20 flex h-14 shrink-0 items-center gap-1.5 border-b bg-background pr-3 pl-2 md:h-16 md:gap-3 md:px-8"
  style="view-transition-name: header;"
>
  <Sidebar.Trigger class="size-9 md:-ml-2" />

  <a href="/" class="flex items-center gap-2 font-bold tracking-tight md:hidden">
    <Activity class="size-5 text-primary" />
    Stamon
  </a>

  <Breadcrumb.Root class="hidden min-w-0 md:block">
    <Breadcrumb.List class="flex-nowrap text-sm">
      {#each trail as crumb, idx (idx)}
        {#if idx > 0}
          <Breadcrumb.Separator>/</Breadcrumb.Separator>
        {/if}
        <Breadcrumb.Item class="min-w-0">
          {#if idx === trail.length - 1}
            <Breadcrumb.Page class="truncate font-semibold">{crumb.label}</Breadcrumb.Page>
          {:else}
            <Breadcrumb.Link href={crumb.href} class="truncate">{crumb.label}</Breadcrumb.Link>
          {/if}
        </Breadcrumb.Item>
      {/each}
    </Breadcrumb.List>
  </Breadcrumb.Root>

  <div class="flex-1"></div>

  <ServiceSearch bind:ref={searchInput} class="hidden w-70 md:block" />

  <Dialog.Root bind:open={searchOpen}>
    <Dialog.Trigger>
      {#snippet child({ props })}
        <Button {...props} variant="ghost" size="icon" class="md:hidden">
          <Search class="size-[18px] text-muted-foreground" />
          <span class="sr-only">Search services</span>
        </Button>
      {/snippet}
    </Dialog.Trigger>
    <Dialog.Content class="top-16 translate-y-0 p-4">
      <Dialog.Title class="sr-only">Search services</Dialog.Title>
      <ServiceSearch inline onnavigate={() => (searchOpen = false)} />
    </Dialog.Content>
  </Dialog.Root>

  <div class="hidden md:block">
    <NotificationPopover />
  </div>

  <DropdownMenu.Root>
    <DropdownMenu.Trigger>
      {#snippet child({ props })}
        <Button {...props} variant="outline" size="icon" class="relative hidden md:inline-flex">
          <Sun
            class="size-[18px] rotate-0 scale-100 text-muted-foreground transition-all dark:-rotate-90 dark:scale-0"
          />
          <Moon
            class="absolute size-[18px] rotate-90 scale-0 text-muted-foreground transition-all dark:rotate-0 dark:scale-100"
          />
          <span class="sr-only">Toggle theme</span>
        </Button>
      {/snippet}
    </DropdownMenu.Trigger>
    <DropdownMenu.Content align="end">
      <DropdownMenu.Item onSelect={() => setMode('light')}>Light</DropdownMenu.Item>
      <DropdownMenu.Item onSelect={() => setMode('dark')}>Dark</DropdownMenu.Item>
      <DropdownMenu.Item onSelect={() => resetMode()}>System</DropdownMenu.Item>
    </DropdownMenu.Content>
  </DropdownMenu.Root>

  <div class="hidden h-6 w-px bg-border md:block"></div>

  <DropdownMenu.Root>
    <DropdownMenu.Trigger>
      {#snippet child({ props })}
        <button
          {...props}
          type="button"
          class="flex items-center gap-2.5 rounded-lg p-0.5 text-left md:pl-1"
        >
          <Avatar.Root class="size-8">
            <Avatar.Fallback class="bg-primary text-xs font-bold text-primary-foreground">
              {initials}
            </Avatar.Fallback>
          </Avatar.Root>
          <span class="hidden flex-col md:flex">
            <span class="text-sm leading-tight font-semibold">{$user?.username}</span>
            <span class="text-xs leading-tight text-muted-foreground capitalize">{$user?.role}</span>
          </span>
          <ChevronDown class="hidden size-4 text-muted-foreground md:block" />
          <span class="sr-only">Toggle user menu</span>
        </button>
      {/snippet}
    </DropdownMenu.Trigger>
    <DropdownMenu.Content align="end" class="min-w-48">
      <DropdownMenu.Label>
        <div class="flex flex-col space-y-1">
          <p class="text-sm leading-none font-medium">{$user?.username}</p>
          <p class="text-xs leading-none text-muted-foreground capitalize">{$user?.role}</p>
        </div>
      </DropdownMenu.Label>
      <DropdownMenu.Separator />
      <DropdownMenu.Item href="/settings/">
        <UserRoundCog class="mr-2 size-4" />
        Account
      </DropdownMenu.Item>
      <DropdownMenu.Item
        href={$user?.role === 'admin' ? '/settings/general/' : '/settings/appearance/'}
      >
        <Settings class="mr-2 size-4" />
        Settings
      </DropdownMenu.Item>
      <DropdownMenu.Item class="md:hidden" onSelect={() => setMode('light')}>
        <Sun class="mr-2 size-4" />
        Light mode
      </DropdownMenu.Item>
      <DropdownMenu.Item class="md:hidden" onSelect={() => setMode('dark')}>
        <Moon class="mr-2 size-4" />
        Dark mode
      </DropdownMenu.Item>
      <DropdownMenu.Separator />
      <DropdownMenu.Item onSelect={logout}>
        <LogOut class="mr-2 size-4" />
        Logout
      </DropdownMenu.Item>
    </DropdownMenu.Content>
  </DropdownMenu.Root>
</header>
