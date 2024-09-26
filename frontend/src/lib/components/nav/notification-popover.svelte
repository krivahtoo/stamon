<script>
  import * as Popover from '$lib/components/ui/popover/index';
  import * as Card from '$lib/components/ui/card/index.js';
  import Bell from 'lucide-svelte/icons/bell';
  import BellRing from 'lucide-svelte/icons/bell-ring';
  import Check from 'lucide-svelte/icons/check';
  import { Button } from '$lib/components/ui/button/index.js';
  import { Switch } from '$lib/components/ui/switch/index.js';
  import notifications from '$lib/store/notifications.js';
</script>

<Popover.Root>
  <Popover.Trigger asChild let:builder>
    <Button builders={[builder]} variant="outline" size="icon" class="ml-auto h-8 w-8">
      <Bell class="h-4 w-4" />
      <span class="sr-only">Toggle notifications</span>
    </Button>
  </Popover.Trigger>
  <Popover.Content>
    <Card.Root class="w-fit">
      <Card.Header>
        <Card.Title>Notifications</Card.Title>
        <Card.Description>You have {$notifications.length} unread messages.</Card.Description>
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
          {#each $notifications as notification, idx (idx)}
            <div class="mb-4 grid grid-cols-[25px_1fr] items-start pb-4 last:mb-0 last:pb-0">
              <span class="flex h-2 w-2 translate-y-1 rounded-full bg-sky-500" />
              <div class="space-y-1">
                <p class="text-sm font-medium leading-none">
                  {notification.title}
                </p>
                <p class="text-sm text-muted-foreground">
                  {notification.message}
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
