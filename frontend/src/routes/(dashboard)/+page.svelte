<script>
  import Activity from 'lucide-svelte/icons/activity';
  import CreditCard from 'lucide-svelte/icons/credit-card';
  import Timer from 'lucide-svelte/icons/timer';
  import Users from 'lucide-svelte/icons/users';
  import * as Card from '$lib/components/ui/card/index.js';
  import * as Select from '$lib/components/ui/select/index.js';
  import { Metric } from '$lib/components/chart/index.js';
</script>

<svelte:head>
  <title>Home</title>
  <meta name="description" content="Svelte demo app" />
</svelte:head>

<div class="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
  <Card.Root class="bg-green-500/10 dark:bg-green-500/20 border-green-500/50">
    <Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
      <Card.Title class="text-sm font-medium">Up Monitors</Card.Title>
      <Timer class="h-4 w-4 text-muted-foreground" />
    </Card.Header>
    <Card.Content>
      <div class="text-2xl font-bold">12</div>
      <p class="text-xs text-muted-foreground">+20.1% from last month</p>
    </Card.Content>
  </Card.Root>
  <Card.Root class="bg-red-500/10 dark:bg-red-500/20 border-red-500/50">
    <Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
      <Card.Title class="text-sm font-medium">Down</Card.Title>
      <Users class="h-4 w-4 text-muted-foreground" />
    </Card.Header>
    <Card.Content>
      <div class="text-2xl font-bold">3</div>
      <p class="text-xs text-muted-foreground">+180.1% from last month</p>
    </Card.Content>
  </Card.Root>
  <Card.Root class="bg-yellow-500/10 dark:bg-yellow-500/20 border-yellow-500/50">
    <Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
      <Card.Title class="text-sm font-medium">Pending</Card.Title>
      <CreditCard class="h-4 w-4 text-muted-foreground" />
    </Card.Header>
    <Card.Content>
      <div class="text-2xl font-bold">1</div>
      <p class="text-xs text-muted-foreground">+19% from last month</p>
    </Card.Content>
  </Card.Root>
  <Card.Root class="bg-primary/10 dark:bg-primary/20 border-primary/50">
    <Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
      <Card.Title class="text-sm font-medium">Paused Monitors</Card.Title>
      <Activity class="h-4 w-4 text-muted-foreground" />
    </Card.Header>
    <Card.Content>
      <div class="text-2xl font-bold">3</div>
      <p class="text-xs text-muted-foreground">+201 since last hour</p>
    </Card.Content>
  </Card.Root>
</div>

<Card.Root>
  <Card.Header class="flex">
    <Card.Title>Ping</Card.Title>
    <Card.Description class="flex flex-row">
      <div class="">
        Your excercise minutes are ahead of where you normally are.
      </div>
      <Select.Root>
        <Select.Trigger id="status" aria-label="Select status" class="ml-auto w-auto pr-2">
          <Select.Value placeholder="Select status" />
        </Select.Trigger>
        <Select.Content class="w-auto">
          <Select.Item value="draft" label="Draft">Draft</Select.Item>
          <Select.Item value="published" label="Active">Active</Select.Item>
          <Select.Item value="archived" label="Archived">Archived</Select.Item>
        </Select.Content>
      </Select.Root>
    </Card.Description>
  </Card.Header>
  <Card.Content class="pb-4">
    <div class="h-fit">
      <Metric />
    </div>
  </Card.Content>
</Card.Root>