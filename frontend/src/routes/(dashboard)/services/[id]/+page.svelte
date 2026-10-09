<script>
  import Activity from 'lucide-svelte/icons/activity';
  import CalendarRange from 'lucide-svelte/icons/calendar-range';
  import HeartPulse from 'lucide-svelte/icons/heart-pulse';
  import Timer from 'lucide-svelte/icons/timer';
  import ChevronLeft from 'lucide-svelte/icons/chevron-left';
  import * as Card from '$lib/components/ui/card/index.js';
  import * as Select from '$lib/components/ui/select/index.js';
  import { Metric } from '$lib/components/chart/index.js';
  import { Separator } from '$lib/components/ui/separator/index.js';
  import { Button } from '$lib/components/ui/button/index.js';
  import { Input } from '$lib/components/ui/input/index.js';
  import { Badge } from '$lib/components/ui/badge/index.js';
  import { writable } from 'svelte/store';
  import { onMount } from 'svelte';
  import { toast } from 'svelte-sonner';
  import ChannelPicker from '$lib/components/channel-picker.svelte';
  import user from '$lib/store/user.js';
  import { cfetch, pushUrl } from '$lib/utils.js';
  import { statuses } from '$lib/data/table.js';

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

  /**
   * Channels picked for this service, once loaded.
   * @type {number[]}
   */
  let channelIds = [];
  let channelsLoaded = false;

  /**
   * @typedef {Object} Uptime
   * @property {number | null} percent
   * @property {number | null} avg_latency
   * @property {number} checks
   */

  /** @type {{ uptime_24h: Uptime, uptime_7d: Uptime, uptime_30d: Uptime, status_since: string | null } | null} */
  let stats = null;

  /** @param {number | null | undefined} percent */
  function formatPercent(percent) {
    return percent == null ? '–' : `${percent.toFixed(percent === 100 ? 0 : 2)}%`;
  }

  /** @param {string} time */
  function timeAgo(time) {
    const seconds = Math.max(0, (Date.now() - new Date(time).getTime()) / 1000);
    const units = /** @type {const} */ ([
      ['day', 86400],
      ['hour', 3600],
      ['minute', 60]
    ]);
    for (const [unit, size] of units) {
      if (seconds >= size) {
        const n = Math.floor(seconds / size);
        return `${n} ${unit}${n === 1 ? '' : 's'} ago`;
      }
    }
    return 'just now';
  }

  $: status = statuses.find((s) => s.value === data.service.last_status);
  $: canEdit = $user?.role === 'admin' || $user?.role === 'editor';

  onMount(async () => {
    logs.set(data.logs);
    cfetch(`/services/${data.service.id}/stats`, { credentials: 'same-origin' }).then(
      async (res) => {
        if (res.ok) stats = (await res.json()).stats;
      }
    );
    const res = await cfetch(`/services/${data.service.id}/channels`, {
      credentials: 'same-origin'
    });
    if (res.ok) channelIds = (await res.json()).channel_ids;
    channelsLoaded = true;
  });

  /** @type {string[]} */
  let tags = data.service.tags ?? [];
  let tagsText = tags.join(', ');

  async function saveTags() {
    const res = await cfetch(`/services/${data.service.id}`, {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      credentials: 'same-origin',
      body: JSON.stringify({ tags: tagsText.split(',') })
    });
    const body = await res.json().catch(() => null);
    if (!res.ok) {
      toast.error(`Error: ${body?.message ?? body?.error ?? res.statusText}`);
      return;
    }
    // Show the tags as the server cleaned them up.
    const reload = await cfetch(`/services/${data.service.id}`, { credentials: 'same-origin' });
    if (reload.ok) tags = (await reload.json()).service.tags;
    tagsText = tags.join(', ');
    toast.success('Tags saved');
  }

  async function saveChannels() {
    const res = await cfetch(`/services/${data.service.id}/channels`, {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      credentials: 'same-origin',
      body: JSON.stringify({ channel_ids: channelIds })
    });
    const body = await res.json().catch(() => null);
    if (res.ok) {
      toast.success('Alert channels saved');
    } else {
      toast.error(`Error: ${body?.message ?? body?.error ?? res.statusText}`);
    }
  }
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
    {#if tags.length > 0}
      <div class="flex flex-wrap gap-1 pt-1">
        {#each tags as tag}
          <Badge variant="outline" class="font-normal">{tag}</Badge>
        {/each}
      </div>
    {/if}
  </div>
</div>
<Separator class="my-1" />
{#if data.service.config?.type === 'push'}
  <Card.Root>
    <Card.Header class="pb-2">
      <Card.Title class="text-sm font-medium">Push URL</Card.Title>
      <Card.Description>
        Call this URL (GET or POST) at least every {data.service.interval}s. Add
        <code>?status=down&amp;msg=...</code> to report a failure.
      </Card.Description>
    </Card.Header>
    <Card.Content>
      <Input readonly value={pushUrl(data.service.config.token)} class="font-mono" />
    </Card.Content>
  </Card.Root>
{/if}
<div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
  <Card.Root>
    <Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
      <Card.Title class="text-sm font-medium">Average Latency</Card.Title>
      <Timer class="h-4 w-4 text-muted-foreground" />
    </Card.Header>
    <Card.Content>
      <div class="text-2xl font-bold">
        {stats?.uptime_24h.avg_latency == null
          ? '–'
          : `${stats.uptime_24h.avg_latency.toFixed(1)}ms`}
      </div>
      <p class="text-xs text-muted-foreground">
        {stats?.uptime_7d.avg_latency == null
          ? 'Last 24 hours'
          : `${stats.uptime_7d.avg_latency.toFixed(1)}ms over 7 days`}
      </p>
    </Card.Content>
  </Card.Root>
  <Card.Root>
    <Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
      <Card.Title class="text-sm font-medium">Uptime (24h)</Card.Title>
      <Activity class="h-4 w-4 text-muted-foreground" />
    </Card.Header>
    <Card.Content>
      <div class="text-2xl font-bold">{formatPercent(stats?.uptime_24h.percent)}</div>
      <p class="text-xs text-muted-foreground">
        {formatPercent(stats?.uptime_7d.percent)} over 7 days
      </p>
    </Card.Content>
  </Card.Root>
  <Card.Root>
    <Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
      <Card.Title class="text-sm font-medium">Uptime (30d)</Card.Title>
      <CalendarRange class="h-4 w-4 text-muted-foreground" />
    </Card.Header>
    <Card.Content>
      <div class="text-2xl font-bold">{formatPercent(stats?.uptime_30d.percent)}</div>
      <p class="text-xs text-muted-foreground">{stats?.uptime_30d.checks ?? 0} checks</p>
    </Card.Content>
  </Card.Root>
  <Card.Root>
    <Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
      <Card.Title class="text-sm font-medium">Current Status</Card.Title>
      <HeartPulse class="h-4 w-4 text-muted-foreground" />
    </Card.Header>
    <Card.Content>
      <div class="text-2xl font-bold">
        {data.service.active ? (status?.label ?? 'Unknown') : 'Paused'}
      </div>
      <p class="text-xs text-muted-foreground">
        {stats?.status_since ? `Since ${timeAgo(stats.status_since)}` : 'No checks yet'}
      </p>
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

<Card.Root>
  <Card.Header class="pb-2">
    <Card.Title>Alert Channels</Card.Title>
    <Card.Description
      >Where alerts go when {data.service.name} goes down or recovers.</Card.Description
    >
  </Card.Header>
  <Card.Content>
    {#if channelsLoaded}
      <ChannelPicker bind:selected={channelIds} disabled={!canEdit} />
    {/if}
  </Card.Content>
  {#if canEdit}
    <Card.Footer>
      <Button size="sm" on:click={saveChannels}>Save channels</Button>
    </Card.Footer>
  {/if}
</Card.Root>

{#if canEdit}
  <Card.Root>
    <Card.Header class="pb-2">
      <Card.Title>Tags</Card.Title>
      <Card.Description>Labels for grouping and finding this service.</Card.Description>
    </Card.Header>
    <Card.Content>
      <form class="flex gap-2" on:submit|preventDefault={saveTags}>
        <Input bind:value={tagsText} placeholder="Comma-separated, e.g. prod, api" />
        <Button type="submit" size="sm">Save tags</Button>
      </form>
    </Card.Content>
  </Card.Root>
{/if}
