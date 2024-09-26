<script>
  import SidebarNav from './(components)/sidebar-nav.svelte';
  import { Separator } from '$lib/components/ui/separator/index.js';
  import { Button } from '$lib/components/ui/button/index.js';
  import * as Tooltip from '$lib/components/ui/tooltip/index.js';
  import ChevronLeft from 'lucide-svelte/icons/chevron-left';
  import Activity from 'lucide-svelte/icons/activity';
  import Header from '$lib/components/nav/header.svelte';
  import { navItems } from '$lib/data/nav.js';
  import { fly } from 'svelte/transition';

  const sidebarNavItems = [
    {
      title: 'General',
      href: '/settings/'
    },
    {
      title: 'Account',
      href: '/settings/account/'
    },
    {
      title: 'Appearance',
      href: '/settings/appearance/'
    }
  ];
</script>

<div class="flex min-h-screen w-full flex-col bg-muted/40">
  <aside
    in:fly={{ x: -10, duration: 200 }}
    class="fixed inset-y-0 left-0 z-10 hidden w-14 flex-col border-r bg-background md:flex"
  >
    <nav class="flex flex-col items-center gap-4 px-2 sm:py-5">
      <a
        href="/"
        class="group flex h-9 w-9 shrink-0 items-center justify-center gap-2 rounded-full bg-primary text-lg font-semibold text-primary-foreground md:h-8 md:w-8 md:text-base"
      >
        <Activity class="h-4 w-4 transition-all group-hover:scale-125" />
        <span class="sr-only">Stamon</span>
      </a>
      {#each navItems as item, idx (idx)}
        <Tooltip.Root>
          <Tooltip.Trigger asChild let:builder>
            <a
              href={item.path}
              class="flex h-9 w-9 items-center justify-center rounded-lg text-muted-foreground transition-colors hover:text-foreground md:h-8 md:w-8"
              use:builder.action
              {...builder}
            >
              <svelte:component this={item.icon} class="h-5 w-5" />
              <span class="sr-only">{item.name}</span>
            </a>
          </Tooltip.Trigger>
          <Tooltip.Content side="right">{item.name}</Tooltip.Content>
        </Tooltip.Root>
      {/each}
    </nav>
  </aside>

  <div class="flex flex-col md:pl-14">
    <Header {navItems} />

    <div class="space-y-6 p-10 pb-16 md:block">
      <div class="flex items-center gap-4">
        <Button
          variant="ghost"
          size="icon"
          class="h-7 w-7 md:mx-12 lg:mx-16 lg:h-auto lg:w-auto lg:p-2"
          on:click={() => history.back()}
        >
          <ChevronLeft class="h-4 w-4" />
          <span class="sr-only lg:not-sr-only">Back</span>
        </Button>
        <div class="space-y-0.5">
          <h2 class="text-2xl font-bold tracking-tight">Settings</h2>
          <p class="text-muted-foreground">
            Manage your account settings and set e-mail preferences.
          </p>
        </div>
      </div>
      <Separator class="my-6" />
      <div class="flex flex-col space-y-8 md:flex-row lg:space-x-12 lg:space-y-0">
        <aside class="mx-1 md:w-1/5 lg:mx-2 lg:w-1/5">
          <SidebarNav items={sidebarNavItems} />
        </aside>
        <div class="flex-1">
          <slot />
        </div>
      </div>
    </div>
  </div>
</div>
