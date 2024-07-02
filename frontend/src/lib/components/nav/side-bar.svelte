<script>
  import { page } from '$app/stores';

  import { Button } from '$lib/components/ui/button/index.js';
  import { Badge } from '$lib/components/ui/badge/index.js';
  import { Switch } from '$lib/components/ui/switch/index.js';
  import * as Card from '$lib/components/ui/card/index.js';
  import * as Popover from '$lib/components/ui/popover/index';
  import Home from 'lucide-svelte/icons/home';
  import Bell from 'lucide-svelte/icons/bell';
  import {
    Layers3,
    Users,
    LineChart,
    Monitor,
    Activity,
    BellRing,
    Check,
    MessageSquareText
  } from 'lucide-svelte';
  import { onMount } from 'svelte';
  import { toast } from 'svelte-sonner';

  const notifications = [
    {
      title: 'Your call has been confirmed.',
      description: '1 hour ago'
    },
    {
      title: 'You have a new message!',
      description: '1 hour ago'
    },
    {
      title: 'Your subscription is expiring soon!',
      description: '2 hours ago'
    }
  ];

  // Subscribe to the page path
  $: activeUrl = $page?.url?.pathname;
</script>

<div class="hidden border-r dark:border-primary/50 bg-muted/40 md:block">
  <div class="flex h-full max-h-screen flex-col gap-2">
    <div class="flex h-14 items-center border-b dark:border-primary/50 px-4 lg:h-[60px] lg:px-6">
      <a href="/" class="flex items-center gap-2 font-semibold">
        <Activity class="h-6 w-6" />
        <span class="">Stamon</span>
      </a>
      <Popover.Root>
        <Popover.Trigger asChild let:builder>
          <Button builders={[builder]} variant="outline" size="icon" class="ml-auto h-8 w-8">
            <Bell class="h-4 w-4" />
            <span class="sr-only">Toggle notifications</span>
          </Button>
        </Popover.Trigger>
        <Popover.Content>
          <Card.Root class="w-[380px]">
            <Card.Header>
              <Card.Title>Notifications</Card.Title>
              <Card.Description>You have {notifications.length} unread messages.</Card.Description>
            </Card.Header>
            <Card.Content class="grid gap-4">
              <div class=" flex items-center space-x-4 rounded-md border p-4">
                <BellRing />
                <div class="flex-1 space-y-1">
                  <p class="text-sm font-medium leading-none">Push Notifications</p>
                  <p class="text-sm text-muted-foreground">Send notifications to device.</p>
                </div>
                <Switch />
              </div>
              <div>
                {#each notifications as notification, idx (idx)}
                  <div class="mb-4 grid grid-cols-[25px_1fr] items-start pb-4 last:mb-0 last:pb-0">
                    <span class="flex h-2 w-2 translate-y-1 rounded-full bg-sky-500" />
                    <div class="space-y-1">
                      <p class="text-sm font-medium leading-none">
                        {notification.title}
                      </p>
                      <p class="text-sm text-muted-foreground">
                        {notification.description}
                      </p>
                    </div>
                  </div>
                {/each}
              </div>
            </Card.Content>
            <Card.Footer>
              <Button class="w-full">
                <Check class="mr-2 h-4 w-4" /> Mark all as read
              </Button>
            </Card.Footer>
          </Card.Root>
        </Popover.Content>
      </Popover.Root>
    </div>
    <div class="flex-1">
      <nav class="grid items-start px-2 text-sm font-medium lg:px-4">
        <a
          href="/"
          class="flex items-center gap-3 rounded-lg px-3 py-2 transition-all hover:text-primary {activeUrl ===
          '/'
            ? 'bg-background/40 text-primary'
            : 'text-muted-foreground'}"
        >
          <Home class="h-4 w-4" />
          Dashboard
        </a>
        <a
          href="/monitors"
          class="flex items-center gap-3 rounded-lg px-3 py-2 transition-all hover:text-primary {activeUrl ===
          '/monitors/'
            ? 'bg-background/40 text-primary'
            : 'text-muted-foreground'}"
        >
          <Layers3 class="h-4 w-4" />
          Monitors
          <Badge class="ml-auto flex h-6 w-6 shrink-0 items-center justify-center rounded-full">
            6
          </Badge>
        </a>
        <a
          href="/status-pages"
          class="flex items-center gap-3 rounded-lg px-3 py-2 transition-all hover:text-primary {activeUrl ===
          '/status-pages/'
            ? 'bg-background/40 text-primary'
            : 'text-muted-foreground'}"
        >
          <Monitor class="h-4 w-4" />
          Status Pages
        </a>
        <a
          href="/notification"
          class="flex items-center gap-3 rounded-lg px-3 py-2 transition-all hover:text-primary {activeUrl ===
          '/notification/'
            ? 'bg-background/40 text-primary'
            : 'text-muted-foreground'}"
        >
          <MessageSquareText class="h-4 w-4" />
          Notification Providers
        </a>
        <a
          href="/users/"
          class="flex items-center gap-3 rounded-lg px-3 py-2 transition-all hover:text-primary {activeUrl ===
          '/users/'
            ? 'bg-background/40 text-primary'
            : 'text-muted-foreground'}"
        >
          <Users class="h-4 w-4" />
          Team
        </a>
      </nav>
    </div>
    <div class="mt-auto p-4">
      <Card.Root
        data-x-chunk-name="dashboard-02-chunk-0"
        data-x-chunk-description="A card with a call to action"
      >
        <Card.Header class="p-2 pt-0 md:p-4">
          <Card.Title>Upgrade to Pro</Card.Title>
          <Card.Description>
            Unlock all features and get unlimited access to our support team.
          </Card.Description>
        </Card.Header>
        <Card.Content class="p-2 pt-0 md:p-4 md:pt-0">
          <Button
            size="sm"
            class="w-full"
            on:click={() =>
              toast.success('Event has been created', {
                description: 'Sunday, December 03, 2023 at 9:00 AM',
                action: {
                  label: 'Undo',
                  onClick: () => console.info('Undo')
                }
              })}>Upgrade</Button
          >
        </Card.Content>
      </Card.Root>
    </div>
  </div>
</div>