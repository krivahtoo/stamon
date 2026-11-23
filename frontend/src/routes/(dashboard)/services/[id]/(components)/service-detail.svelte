<script>
  import { onMount, untrack } from 'svelte';
  import { goto } from '$app/navigation';
  import { toast } from 'svelte-sonner';
  import ArrowLeft from '@lucide/svelte/icons/arrow-left';
  import Bell from '@lucide/svelte/icons/bell';
  import Copy from '@lucide/svelte/icons/copy';
  import Pause from '@lucide/svelte/icons/pause';
  import Pencil from '@lucide/svelte/icons/pencil';
  import Play from '@lucide/svelte/icons/play';
  import Trash2 from '@lucide/svelte/icons/trash-2';
  import * as Card from '$lib/components/ui/card/index.js';
  import * as Dialog from '$lib/components/ui/dialog/index.js';
  import { Badge } from '$lib/components/ui/badge/index.js';
  import { Button } from '$lib/components/ui/button/index.js';
  import { Input } from '$lib/components/ui/input/index.js';
  import { Callout, StatCard, StatusPill } from '$lib/components/dashboard/index.js';
  import ChannelPicker from '$lib/components/channel-picker.svelte';
  import user from '$lib/store/user.js';
  import { typeLabel } from '$lib/data/checks.js';
  import { deleteService, togglePause } from '$lib/data/service-actions.js';
  import { statuses, StatusValue } from '$lib/data/table.js';
  import { cfetch, formatTime, pushUrl } from '$lib/utils.js';
  import { formatDuration } from '../../../(components)/format.js';
  import ResponseTimeChart from './response-time-chart.svelte';
  import CheckSettings from './check-settings.svelte';
  import CheckHistory from './check-history.svelte';

  /**
   * @typedef {Object} Props
   * @property {import('$lib/store/services').Service} service - As loaded; the page keys
   *   this component on the service id, so a different service starts afresh.
   */

  /** @type {Props} */
  let { service: loaded } = $props();

  // A copy, so pausing can update it in place.
  let service = $state(untrack(() => loaded));

  /**
   * @typedef {Object} Uptime
   * @property {number | null} percent
   * @property {number | null} avg_latency
   * @property {number} checks
   */

  /** @type {{ uptime_24h: Uptime, uptime_7d: Uptime, uptime_30d: Uptime, status_since: string | null } | null} */
  let stats = $state(null);
  /** The message of the latest check, for the down callout. */
  let latestMessage = $state('');

  /** @type {{ id: number, name: string, active: boolean, apply_to_all: boolean, channel_type: string }[]} */
  let channels = $state([]);
  /**
   * Channels picked for this service, once loaded.
   * @type {number[]}
   */
  let channelIds = $state([]);
  let channelsLoaded = $state(false);
  /** @type {string | null} */
  let channelsError = $state(null);
  let managingChannels = $state(false);
  /** The saved channel ids, to go back to when managing is cancelled. */
  let savedChannelIds = $state(/** @type {number[]} */ ([]));

  let tags = $state(untrack(() => loaded.tags ?? []));
  let tagsText = $state('');
  let editingTags = $state(false);

  let deleteDialogOpen = $state(false);

  let status = $derived(statuses.find((s) => s.value === service.last_status));
  let tone = $derived(service.active ? (status?.tone ?? 'neutral') : 'neutral');
  let isDown = $derived(service.active && service.last_status === StatusValue.Down);
  let canEdit = $derived($user?.role === 'admin' || $user?.role === 'editor');
  let config = $derived(/** @type {{ type?: string, token?: string }} */ (service.config));
  let isPush = $derived(config.type === 'push');
  let token = $derived(config.token);

  /** Channels alerted about this service: the picked ones and those for every service. */
  let alerted = $derived(
    channels.filter((c) => channelIds.includes(c.id) || (c.apply_to_all && c.active))
  );

  /** @param {number | null | undefined} percent */
  const formatPercent = (percent) =>
    percent == null ? '–' : `${percent.toFixed(percent === 100 ? 0 : 2)}%`;

  /**
   * @param {number | null | undefined} percent
   * @returns {import('$lib/components/dashboard/tones.js').Tone | undefined}
   */
  const uptimeTone = (percent) =>
    percent == null ? undefined : percent >= 99 ? 'up' : percent >= 95 ? 'pending' : 'down';

  /** @param {string} time */
  const clock = (time) =>
    formatTime(time, { hour: '2-digit', minute: '2-digit', hourCycle: 'h23' });

  let statusContext = $derived.by(() => {
    if (!service.active) return 'Checks are paused';
    if (!stats?.status_since) return 'No checks yet';
    const since = stats.status_since;
    const day = Date.now() - new Date(since).getTime() > 24 * 3600 * 1000;
    const start = day ? formatTime(since, { month: 'short', day: 'numeric' }) : clock(since);
    return `Since ${start} (${formatDuration(since, new Date().toISOString())})`;
  });

  let downText = $derived.by(() => {
    const message = latestMessage || 'The last check failed';
    const since = stats?.status_since;
    if (!since) return `${message}.`;
    return `${message} since ${clock(since)} (${formatDuration(since, new Date().toISOString())}).`;
  });

  onMount(async () => {
    cfetch(`/services/${service.id}/stats`).then(async (res) => {
      if (res.ok) stats = (await res.json()).stats;
    });
    cfetch(`/services/${service.id}/logs?limit=1`).then(async (res) => {
      if (res.ok) latestMessage = (await res.json()).logs[0]?.message ?? '';
    });
    cfetch('/channels').then(async (res) => {
      if (res.ok) channels = (await res.json()).channels;
    });
    // Saving replaces the service's channels, so it stays off until they load.
    try {
      const res = await cfetch(`/services/${service.id}/channels`);
      if (res.ok) {
        channelIds = (await res.json()).channel_ids;
        savedChannelIds = channelIds;
        channelsLoaded = true;
      } else {
        channelsError = `Could not load alert channels (${res.status})`;
      }
    } catch (e) {
      channelsError = `Could not load alert channels: ${e instanceof Error ? e.message : e}`;
    }
  });

  /**
   * @param {string} text
   * @param {string} what
   */
  async function copy(text, what) {
    try {
      await navigator.clipboard.writeText(text);
      toast.success(`${what} copied`);
    } catch {
      toast.error(`Could not copy the ${what.toLowerCase()}`);
    }
  }

  async function pause() {
    try {
      service.active = await togglePause(service);
    } catch {
      // The toast reports failures.
    }
  }

  async function remove() {
    deleteDialogOpen = false;
    try {
      await deleteService(service);
      goto('/services');
    } catch {
      // The toast reports failures.
    }
  }

  function editTags() {
    tagsText = tags.join(', ');
    editingTags = true;
  }

  async function saveTags() {
    const res = await cfetch(`/services/${service.id}`, {
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
    const reload = await cfetch(`/services/${service.id}`);
    if (reload.ok) tags = (await reload.json()).service.tags;
    editingTags = false;
    toast.success('Tags saved');
  }

  async function saveChannels() {
    let res;
    try {
      res = await cfetch(`/services/${service.id}/channels`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        credentials: 'same-origin',
        body: JSON.stringify({ channel_ids: channelIds })
      });
    } catch (e) {
      toast.error(`Error: ${e instanceof Error ? e.message : e}`);
      return;
    }
    const body = await res.json().catch(() => null);
    if (res.ok) {
      savedChannelIds = channelIds;
      managingChannels = false;
      toast.success('Alert channels saved');
    } else {
      toast.error(`Error: ${body?.message ?? body?.error ?? res.statusText}`);
    }
  }
</script>

<div class="flex flex-col gap-6">
  <header class="flex flex-col gap-4 sm:flex-row sm:items-end sm:justify-between">
    <div class="flex min-w-0 flex-col gap-1.5">
      <a
        href="/services"
        class="flex w-fit items-center gap-1.5 text-sm text-muted-foreground hover:text-foreground"
      >
        <ArrowLeft class="size-4" />
        Services
      </a>
      <div class="flex min-w-0 items-center gap-2.5">
        <h1 class="truncate text-2xl font-bold tracking-[-0.015em]">{service.name}</h1>
        <StatusPill {tone}>{service.active ? (status?.label ?? 'Unknown') : 'Paused'}</StatusPill>
      </div>
      <div class="flex flex-wrap items-center gap-2 text-sm text-muted-foreground">
        {#if service.target}
          <button
            type="button"
            class="flex max-w-full min-w-0 items-center gap-1.5 rounded-sm bg-raised px-2 py-0.5 font-mono text-xs text-foreground hover:bg-border"
            onclick={() => copy(/** @type {string} */ (service.target), 'Target')}
          >
            <span class="truncate">{service.target}</span>
            <Copy class="size-3.5 shrink-0 text-muted-foreground" />
            <span class="sr-only">Copy target</span>
          </button>
        {/if}
        <Badge variant="secondary">{typeLabel(service.service_type)}</Badge>
        <span>Every {service.interval}s</span>
      </div>
    </div>
    {#if canEdit}
      <div class="flex gap-2 *:flex-1 sm:shrink-0 sm:*:flex-none">
        <Button variant="outline" onclick={pause}>
          {#if service.active}
            <Pause />
            Pause
          {:else}
            <Play />
            Resume
          {/if}
        </Button>
        <Button variant="outline" href={`/services/${service.id}/edit`}>
          <Pencil />
          Edit
        </Button>
        <Button
          variant="outline"
          size="icon"
          class="flex-none! text-destructive hover:text-destructive"
          onclick={() => (deleteDialogOpen = true)}
        >
          <Trash2 />
          <span class="sr-only">Delete {service.name}</span>
        </Button>
      </div>
    {/if}
  </header>

  <div class="grid grid-cols-2 gap-3 sm:gap-4 lg:grid-cols-4">
    <StatCard
      label="Status"
      value={service.active ? (status?.label ?? 'Unknown') : 'Paused'}
      context={statusContext}
      {tone}
    />
    <StatCard
      label="Avg latency"
      value={stats?.uptime_24h.avg_latency == null
        ? '–'
        : `${Math.round(stats.uptime_24h.avg_latency)} ms`}
      context="Last 24 hours"
    />
    <StatCard
      label="Uptime 24h"
      value={formatPercent(stats?.uptime_24h.percent)}
      context={`${(stats?.uptime_24h.checks ?? 0).toLocaleString()} checks`}
      tone={uptimeTone(stats?.uptime_24h.percent)}
    />
    <StatCard
      label="Uptime 30d"
      value={formatPercent(stats?.uptime_30d.percent)}
      context={`${(stats?.uptime_30d.checks ?? 0).toLocaleString()} checks`}
      tone={uptimeTone(stats?.uptime_30d.percent)}
    />
  </div>

  {#if isPush}
    <Card.Root class="gap-3 p-4 sm:p-5">
      <div class="flex flex-col gap-1">
        <h2 class="font-semibold">Push URL</h2>
        <p class="text-sm text-muted-foreground">
          Call this URL (GET or POST) at least every {service.interval}s. Add
          <code class="font-mono text-xs">?status=down&amp;msg=...</code> to report a failure.
        </p>
      </div>
      {#if !token || token === '********'}
        <p class="text-sm text-muted-foreground">
          Only editors and admins can see this monitor's push URL.
        </p>
      {:else}
        <div class="flex gap-2">
          <Input readonly value={pushUrl(token)} class="font-mono text-xs" aria-label="Push URL" />
          <Button variant="outline" onclick={() => copy(pushUrl(token), 'Push URL')}>
            <Copy />
            Copy
          </Button>
        </div>
      {/if}
    </Card.Root>
  {/if}

  {#if isDown}
    <Callout tone="down" title="{service.name} is down">{downText}</Callout>
  {/if}

  <!-- grid-cols-1 keeps long messages from widening the column past the screen. -->
  <div class="grid grid-cols-1 gap-6 lg:grid-cols-[minmax(0,1fr)_22rem] lg:items-start">
    <div class="lg:col-start-1">
      <ResponseTimeChart serviceId={service.id} />
    </div>

    <aside class="flex flex-col gap-6 lg:col-start-2 lg:row-span-2 lg:row-start-1">
      <CheckSettings bind:service {canEdit} />

      <Card.Root class="gap-0 py-0">
        <div class="flex items-center justify-between gap-2 border-b px-4 py-3 sm:px-5">
          <h2 class="font-semibold">Tags</h2>
          {#if canEdit && !editingTags}
            <Button variant="link" size="sm" class="h-auto p-0" onclick={editTags}>Edit</Button>
          {/if}
        </div>
        <div class="px-4 py-3 sm:px-5">
          {#if editingTags}
            <form
              class="flex flex-col gap-2"
              onsubmit={(e) => {
                e.preventDefault();
                saveTags();
              }}
            >
              <Input
                bind:value={tagsText}
                placeholder="Comma-separated, e.g. prod, api"
                aria-label="Tags"
              />
              <div class="flex justify-end gap-2">
                <Button variant="ghost" size="sm" onclick={() => (editingTags = false)}>
                  Cancel
                </Button>
                <Button type="submit" size="sm">Save tags</Button>
              </div>
            </form>
          {:else if tags.length}
            <div class="flex flex-wrap gap-1">
              {#each tags as tag (tag)}
                <Badge variant="outline" class="font-normal">{tag}</Badge>
              {/each}
            </div>
          {:else}
            <p class="text-sm text-muted-foreground">No tags.</p>
          {/if}
        </div>
      </Card.Root>

      <Card.Root class="gap-0 py-0">
        <div class="flex items-center justify-between gap-2 border-b px-4 py-3 sm:px-5">
          <div class="flex items-center gap-2">
            <h2 class="font-semibold">Alert channels</h2>
            {#if channelsLoaded}
              <span
                class="rounded-full bg-raised px-2 py-px text-xs font-semibold text-muted-foreground tabular-nums"
              >
                {alerted.length}
              </span>
            {/if}
          </div>
          {#if canEdit && channelsLoaded && !managingChannels}
            <Button
              variant="link"
              size="sm"
              class="h-auto p-0"
              onclick={() => (managingChannels = true)}
            >
              Manage
            </Button>
          {/if}
        </div>
        {#if channelsError}
          <p class="px-4 py-3 text-sm text-destructive sm:px-5">{channelsError}</p>
        {:else if managingChannels}
          <div class="flex flex-col gap-3 px-4 py-3 sm:px-5">
            <ChannelPicker bind:selected={channelIds} />
            <div class="flex justify-end gap-2">
              <Button
                variant="ghost"
                size="sm"
                onclick={() => {
                  channelIds = savedChannelIds;
                  managingChannels = false;
                }}
              >
                Cancel
              </Button>
              <Button size="sm" onclick={saveChannels}>Save channels</Button>
            </div>
          </div>
        {:else if channelsLoaded && alerted.length === 0}
          <p class="px-4 py-3 text-sm text-muted-foreground sm:px-5">
            No alerts go out when {service.name} goes down or recovers.
          </p>
        {:else}
          <ul class="divide-y">
            {#each alerted as channel (channel.id)}
              <li class="flex items-center gap-3 px-4 py-2.5 sm:px-5">
                <span class="grid size-8 shrink-0 place-items-center rounded-md bg-raised">
                  <Bell class="size-4 text-muted-foreground" />
                </span>
                <div class="flex min-w-0 flex-col">
                  <span class="truncate text-sm font-semibold">{channel.name}</span>
                  <span class="text-xs text-muted-foreground">
                    {channel.channel_type}{channel.active ? '' : ' · paused'}{channel.apply_to_all
                      ? ' · every service'
                      : ''}
                  </span>
                </div>
              </li>
            {/each}
          </ul>
        {/if}
      </Card.Root>
    </aside>

    <div class="lg:col-start-1">
      <CheckHistory serviceId={service.id} />
    </div>
  </div>
</div>

<Dialog.Root bind:open={deleteDialogOpen}>
  <Dialog.Content class="sm:max-w-[425px]">
    <Dialog.Header>
      <Dialog.Title>Delete service</Dialog.Title>
      <Dialog.Description>
        Are you sure you want to delete "{service.name}"?
      </Dialog.Description>
    </Dialog.Header>
    <Dialog.Footer>
      <Button variant="destructive" onclick={remove}>Yes, delete</Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>
