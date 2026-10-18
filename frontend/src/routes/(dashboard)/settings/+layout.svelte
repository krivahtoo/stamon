<script>
  import { page } from '$app/stores';
  import { cn } from '$lib/utils.js';
  import user from '$lib/store/user.js';

  $: isAdmin = $user?.role === 'admin';
  $: tabs = [
    { name: 'Account', href: '/settings/' },
    { name: 'Appearance', href: '/settings/appearance/' },
    ...(isAdmin ? [{ name: 'General', href: '/settings/general/' }] : [])
  ];
</script>

<div class="space-y-0.5">
  <p class="text-xs font-medium uppercase tracking-wide text-muted-foreground">Management</p>
  <h2 class="text-2xl font-bold tracking-tight">Settings</h2>
  <p class="text-muted-foreground">Your account, how the dashboard looks, and instance defaults.</p>
</div>

<nav class="flex gap-1 overflow-x-auto border-b" aria-label="Settings sections">
  {#each tabs as tab (tab.href)}
    {@const active = $page.url.pathname === tab.href}
    <a
      href={tab.href}
      aria-current={active ? 'page' : undefined}
      class={cn(
        '-mb-px whitespace-nowrap border-b-2 px-3 py-2 text-sm font-medium transition-colors',
        active
          ? 'border-primary text-foreground'
          : 'border-transparent text-muted-foreground hover:text-foreground'
      )}
    >
      {tab.name}
    </a>
  {/each}
</nav>

<div class="mx-auto w-full max-w-2xl pb-4">
  <slot />
</div>
