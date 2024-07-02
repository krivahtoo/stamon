<script>
  import SidebarNav from './(components)/sidebar-nav.svelte';
  import { Separator } from '$lib/components/ui/separator/index.js';
  import { Button } from '$lib/components/ui/button/index.js';
  import { ChevronLeft } from 'lucide-svelte';
  import Header from '$lib/components/nav/header.svelte';

  const sidebarNavItems = [
    {
      title: 'General',
      href: '/settings'
    },
    {
      title: 'Account',
      href: '/settings/account'
    },
    {
      title: 'Appearance',
      href: '/settings/appearance'
    }
  ];
</script>

<Header />

<div class="space-y-6 p-10 pb-16 md:block">
  <div class="flex items-center gap-4">
    <Button variant="ghost" size="icon" class="h-7 w-7 md:mx-12 lg:mx-16 lg:p-2 lg:w-auto lg:h-auto" on:click={() => history.back()}>
      <ChevronLeft class="h-4 w-4" />
      <span class="sr-only lg:not-sr-only">Back</span>
    </Button>
    <div class="space-y-0.5">
      <h2 class="text-2xl font-bold tracking-tight">Settings</h2>
      <p class="text-muted-foreground">Manage your account settings and set e-mail preferences.</p>
    </div>
  </div>
  <Separator class="my-6" />
  <div class="flex flex-col space-y-8 md:flex-row lg:space-x-12 lg:space-y-0">
    <aside class="-mx-4 md:w-1/5 lg:mx-2 lg:w-1/5">
      <SidebarNav items={sidebarNavItems} />
    </aside>
    <div class="flex-1">
      <slot />
    </div>
  </div>
</div>