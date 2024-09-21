<script>
  import Activity from 'lucide-svelte/icons/activity';
  import CreditCard from 'lucide-svelte/icons/credit-card';
  import Timer from 'lucide-svelte/icons/timer';
  import Users from 'lucide-svelte/icons/users';
  import ChevronLeft from 'lucide-svelte/icons/chevron-left';
  import * as Card from '$lib/components/ui/card/index.js';
  import * as Select from '$lib/components/ui/select/index.js';
  import { Metric } from '$lib/components/chart/index.js';
  import { Separator } from '$lib/components/ui/separator/index.js';
  import { Button } from '$lib/components/ui/button/index.js';
  import { writable } from 'svelte/store';
  import { onMount } from 'svelte';

  /**
   * @typedef {Object} Log
   * @property {number} service_id - The id of the service.
   * @property {number} status - The status.
   * @property {string} time - Timestamp.
   * @property {number} duration - Time taken.
   */

  /** @type {import('svelte/store').Writable<Log[]>} */
  let logs = writable([]);

  /** @type {import('./$types').PageData} */
  export let data;

  onMount(() => {
    logs.set(data.logs);
  })
</script>

<div class="flex items-center">
  <Button
    variant="ghost"
    size="icon"
    class="h-7 w-7 md:mx-6 lg:mx-10 lg:h-auto lg:w-auto lg:p-2"
    on:click={() => history.back()}
  >
    <ChevronLeft class="h-4 w-4" />
    <span class="sr-only lg:not-sr-only">Back</span>
  </Button>
  <div class="space-y-0.5">
    <h2 class="text-2xl font-bold tracking-tight">{data.service.name}</h2>
    <p class="text-muted-foreground">Below is an overview of {data.service.name} service.</p>
  </div>
</div>
<Separator class="my-1" />
<div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
  <Card.Root>
    <Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
      <Card.Title class="text-sm font-medium">Avarage Latency</Card.Title>
      <Timer class="h-4 w-4 text-muted-foreground" />
    </Card.Header>
    <Card.Content>
      <div class="text-2xl font-bold">131.89ms</div>
      <p class="text-xs text-muted-foreground">+20.1% from last month</p>
    </Card.Content>
  </Card.Root>
  <Card.Root>
    <Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
      <Card.Title class="text-sm font-medium">Uptime</Card.Title>
      <Users class="h-4 w-4 text-muted-foreground" />
    </Card.Header>
    <Card.Content>
      <div class="text-2xl font-bold">99.9%</div>
      <p class="text-xs text-muted-foreground">99.1% last month</p>
    </Card.Content>
  </Card.Root>
  <Card.Root>
    <Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
      <Card.Title class="text-sm font-medium">Current Status</Card.Title>
      <CreditCard class="h-4 w-4 text-muted-foreground" />
    </Card.Header>
    <Card.Content>
      <div class="text-2xl font-bold">Up</div>
      <p class="text-xs text-muted-foreground">Last change a month ago</p>
    </Card.Content>
  </Card.Root>
  <Card.Root>
    <Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
      <Card.Title class="text-sm font-medium">Active Now</Card.Title>
      <Activity class="h-4 w-4 text-muted-foreground" />
    </Card.Header>
    <Card.Content>
      <div class="text-2xl font-bold">+573</div>
      <p class="text-xs text-muted-foreground">+201 since last hour</p>
    </Card.Content>
  </Card.Root>
</div>

<Card.Root>
  <Card.Header class="flex">
    <Card.Title>Ping</Card.Title>
    <Card.Description class="flex flex-row">
      <div class="">Avarage latency for the last few requests.</div>
      <Select.Root>
        <Select.Trigger id="status" aria-label="Select range" class="ml-auto w-auto pr-2">
          <Select.Value placeholder="Select range" />
        </Select.Trigger>
        <Select.Content class="w-auto">
          <Select.Item value="draft" label="Draft">6 hours</Select.Item>
          <Select.Item value="published" label="Active">12 hours</Select.Item>
          <Select.Item value="archived" label="Archived">24 hours</Select.Item>
        </Select.Content>
      </Select.Root>
    </Card.Description>
  </Card.Header>
  <Card.Content class="pb-4">
    <div class="h-fit w-11/12 lg:w-full">
      <Metric data={logs} interval={data.service.interval} />
    </div>
  </Card.Content>
</Card.Root>