<script>
  import {
    CircleUser,
    Menu,
    Search,
    Sun,
    Moon,
    Languages,
    Settings,
    LogOut,
    UserRoundCog
  } from 'lucide-svelte';
  import Activity from 'lucide-svelte/icons/activity';

  import { resetMode, setMode } from 'mode-watcher';
  import { Button } from '$lib/components/ui/button/index.js';
  import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
  import { Input } from '$lib/components/ui/input/index.js';
  import * as Sheet from '$lib/components/ui/sheet/index.js';
  import * as Card from '$lib/components/ui/card/index.js';
  import user from '$lib/store/user.js';
  import { token } from '$lib/store/auth.js';
  import { cfetch } from '$lib/utils.js';
  import { goto } from '$app/navigation';

  /**
   * @typedef {Object} NavItem
   * @property {string} name - Name to show in the navbar
   * @property {string} path - navication item path
   * @property {import('svelte').ComponentType} icon - navication item icon
   */

  /** @type {NavItem[]} */
  export let navItems = [];

  function logout() {
    cfetch('/logout').finally(() => {
      token.set(null);
      user.set(null);
      goto('/login');
    });
  }
</script>

<header
  class="flex h-14 items-center gap-4 border-b bg-muted/40 px-4 dark:border-primary/50 lg:h-[60px] lg:px-6"
>
  <Sheet.Root>
    <Sheet.Trigger asChild let:builder>
      <Button variant="outline" size="icon" class="mr-auto shrink-0 md:hidden" builders={[builder]}>
        <Menu class="h-5 w-5" />
        <span class="sr-only">Toggle navigation menu</span>
      </Button>
    </Sheet.Trigger>
    <Sheet.Content side="left" class="flex flex-col">
      <nav class="grid gap-2 text-lg font-medium">
        <a href="/" class="flex items-center gap-2 text-lg font-semibold">
          <Activity class="h-6 w-6" />
          <span class="sr-only">Stamon</span>
        </a>
        {#each navItems as item, idx (idx)}
          <a
            href={item.path}
            class="mx-[-0.65rem] flex items-center gap-4 rounded-xl px-3 py-2 text-muted-foreground hover:text-foreground"
          >
            <svelte:component this={item.icon} class="h-5 w-5" />
            {item.name}
          </a>
        {/each}
      </nav>
      <div class="mt-auto">
        <Card.Root>
          <Card.Header>
            <Card.Title>Upgrade to Pro</Card.Title>
            <Card.Description>
              Unlock all features and get unlimited access to our support team.
            </Card.Description>
          </Card.Header>
          <Card.Content>
            <Button size="sm" class="w-full">Upgrade</Button>
          </Card.Content>
        </Card.Root>
      </div>
    </Sheet.Content>
  </Sheet.Root>
  <div class="hidden w-full flex-1 sm:block">
    <form>
      <div class="relative">
        <Search class="absolute left-2.5 top-2.5 h-4 w-4 text-muted-foreground" />
        <Input
          type="search"
          placeholder="Search monitors..."
          class="w-full appearance-none bg-background pl-8 shadow-none md:w-2/3 lg:w-1/3"
        />
      </div>
    </form>
  </div>
  <DropdownMenu.Root>
    <DropdownMenu.Trigger asChild let:builder>
      <Button builders={[builder]} variant="link" size="icon" class="hidden sm:flex">
        <Languages class="h-5 w-5" />
        <span class="sr-only">Toggle user menu</span>
      </Button>
    </DropdownMenu.Trigger>
    <DropdownMenu.Content align="end">
      <DropdownMenu.Label>Select language</DropdownMenu.Label>
      <DropdownMenu.Separator />
      <DropdownMenu.Item>English</DropdownMenu.Item>
      <DropdownMenu.Item>French</DropdownMenu.Item>
    </DropdownMenu.Content>
  </DropdownMenu.Root>
  <DropdownMenu.Root>
    <DropdownMenu.Trigger asChild let:builder>
      <Button builders={[builder]} variant="link" size="icon">
        <Sun
          class="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0"
        />
        <Moon
          class="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100"
        />
        <span class="sr-only">Toggle theme</span>
      </Button>
    </DropdownMenu.Trigger>
    <DropdownMenu.Content align="end" class="dark:border-primary/50">
      <DropdownMenu.Item on:click={() => setMode('light')}>Light</DropdownMenu.Item>
      <DropdownMenu.Item on:click={() => setMode('dark')}>Dark</DropdownMenu.Item>
      <DropdownMenu.Item on:click={() => resetMode()}>System</DropdownMenu.Item>
    </DropdownMenu.Content>
  </DropdownMenu.Root>
  <DropdownMenu.Root>
    <DropdownMenu.Trigger asChild let:builder>
      <Button builders={[builder]} variant="secondary" size="icon" class="rounded-full">
        <CircleUser class="h-5 w-5" />
        <span class="sr-only">Toggle user menu</span>
      </Button>
    </DropdownMenu.Trigger>
    <DropdownMenu.Content align="end">
      <DropdownMenu.Label>
        <div class="flex flex-col space-y-1">
          <p class="text-sm font-medium leading-none">{$user?.username}</p>
          <p class="text-xs leading-none text-muted-foreground">{$user?.role}</p>
        </div>
      </DropdownMenu.Label>
      <DropdownMenu.Separator />
      <DropdownMenu.Item href="/settings/account">
        <UserRoundCog class="text-foreground-alt mr-2 size-5" />
        Account
      </DropdownMenu.Item>
      <DropdownMenu.Item href="/settings">
        <Settings class="mr-2 size-5 text-foreground" />
        Settings
      </DropdownMenu.Item>
      <DropdownMenu.Separator />
      <DropdownMenu.Item on:click={logout}>
        <LogOut class="text-foreground-alt mr-2 size-5" />
        Logout
      </DropdownMenu.Item>
    </DropdownMenu.Content>
  </DropdownMenu.Root>
</header>
