<script>
  import { onMount } from 'svelte';
  import { toast } from 'svelte-sonner';

  import { Badge } from '$lib/components/ui/badge/index.js';
  import { Button } from '$lib/components/ui/button/index.js';
  import * as Card from '$lib/components/ui/card/index.js';
  import * as Dialog from '$lib/components/ui/dialog/index.js';
  import { Switch } from '$lib/components/ui/switch/index.js';
  import * as Table from '$lib/components/ui/table/index.js';
  import user from '$lib/store/user.js';
  import { cfetch } from '$lib/utils.js';
  import ChannelForm from './channel-form.svelte';

  /**
   * @typedef {Object} ChannelSummary
   * @property {number} id
   * @property {string} name
   * @property {boolean} active
   * @property {boolean} apply_to_all
   * @property {string} channel_type
   */

  /**
   * @typedef {Object} Delivery
   * @property {number} id
   * @property {string | null} service_name
   * @property {string | null} channel_name
   * @property {string} title
   * @property {string | null} message
   * @property {'sent' | 'failed'} status
   * @property {string | null} error
   * @property {string} sent_at
   */

  /** @type {Record<string, string>} */
  const typeLabels = {
    webhook: 'Webhook',
    slack: 'Slack',
    discord: 'Discord',
    telegram: 'Telegram',
    ntfy: 'ntfy',
    email: 'Email'
  };

  /** @type {ChannelSummary[]} */
  let channels = [];
  /** @type {Delivery[]} */
  let history = [];
  let loading = true;

  let formOpen = false;
  /**
   * The full channel being edited, with its config, or null when adding.
   * @type {(ChannelSummary & { config: Record<string, any> }) | null}
   */
  let editing = null;
  /** @type {ChannelSummary | null} */
  let deleting = null;

  $: isAdmin = $user?.role === 'admin';

  /**
   * Fetch JSON from the API, showing an error toast on failure.
   * @param {string} path
   * @param {RequestInit} [options]
   */
  async function request(path, options) {
    const res = await cfetch(path, { credentials: 'same-origin', ...options });
    const data = await res.json().catch(() => null);
    if (!res.ok) throw new Error(data?.message ?? data?.error ?? res.statusText);
    return data;
  }

  async function load() {
    try {
      const [channelData, historyData] = await Promise.all([
        request('/channels'),
        request('/notifications?limit=50')
      ]);
      channels = channelData.channels;
      history = historyData.notifications;
    } catch (e) {
      toast.error(`Could not load notifications: ${e instanceof Error ? e.message : e}`);
    } finally {
      loading = false;
    }
  }

  onMount(load);

  function add() {
    editing = null;
    formOpen = true;
  }

  /** @param {ChannelSummary} channel */
  async function edit(channel) {
    try {
      editing = (await request(`/channels/${channel.id}`)).channel;
      formOpen = true;
    } catch (e) {
      toast.error(`${e instanceof Error ? e.message : e}`);
    }
  }

  /**
   * @param {ChannelSummary} channel
   * @param {boolean} active
   */
  async function setActive(channel, active) {
    try {
      await request(`/channels/${channel.id}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ active })
      });
      toast.success(`Channel "${channel.name}" ${active ? 'resumed' : 'paused'}`);
    } catch (e) {
      toast.error(`${e instanceof Error ? e.message : e}`);
    }
    await load();
  }

  /** @param {ChannelSummary} channel */
  function test(channel) {
    toast.promise(request(`/channels/${channel.id}/test`, { method: 'POST' }).finally(load), {
      loading: `Sending a test alert to "${channel.name}"…`,
      success: 'Test alert sent',
      error: (e) => `Test failed: ${e instanceof Error ? e.message : e}`
    });
  }

  async function confirmDelete() {
    if (!deleting) return;
    const channel = deleting;
    deleting = null;
    try {
      await request(`/channels/${channel.id}`, { method: 'DELETE' });
      toast.success(`Channel "${channel.name}" deleted`);
    } catch (e) {
      toast.error(`${e instanceof Error ? e.message : e}`);
    }
    await load();
  }

  /** @param {string} time */
  function formatTime(time) {
    return new Date(time).toLocaleString();
  }
</script>

<div class="flex items-center justify-between">
  <div class="space-y-0.5">
    <h2 class="text-2xl font-bold tracking-tight">Notification Providers</h2>
    <p class="text-muted-foreground">
      Channels that receive an alert when a service goes down or comes back up.
    </p>
  </div>
  {#if isAdmin}
    <Button on:click={add}>Add channel</Button>
  {/if}
</div>

{#if !loading && channels.length === 0}
  <section
    class="flex min-h-[200px] flex-1 items-center justify-center rounded-lg border border-dashed shadow-sm"
  >
    <div class="flex flex-col items-center gap-1 p-6 text-center">
      <h3 class="text-xl font-bold tracking-tight">You have no notification channels</h3>
      <p class="text-sm text-muted-foreground">
        {#if isAdmin}
          Add a channel to start receiving status alerts.
        {:else}
          Ask an admin to add a channel to start receiving status alerts.
        {/if}
      </p>
      {#if isAdmin}
        <Button class="mt-4" on:click={add}>Add channel</Button>
      {/if}
    </div>
  </section>
{:else}
  <div class="grid gap-4 md:grid-cols-2 xl:grid-cols-3">
    {#each channels as channel (channel.id)}
      <Card.Root class={channel.active ? '' : 'opacity-60'}>
        <Card.Header class="flex flex-row items-start justify-between space-y-0 pb-2">
          <div class="space-y-1">
            <Card.Title class="text-base">{channel.name}</Card.Title>
            <div class="flex flex-wrap gap-1">
              <Badge variant="secondary"
                >{typeLabels[channel.channel_type] ?? channel.channel_type}</Badge
              >
              {#if channel.apply_to_all}
                <Badge variant="outline">All services</Badge>
              {/if}
              {#if !channel.active}
                <Badge variant="outline">Paused</Badge>
              {/if}
            </div>
          </div>
          {#if isAdmin}
            <Switch
              checked={channel.active}
              onCheckedChange={(/** @type {boolean} */ active) => setActive(channel, active)}
              aria-label={`${channel.active ? 'Pause' : 'Resume'} ${channel.name}`}
            />
          {/if}
        </Card.Header>
        {#if isAdmin}
          <Card.Footer class="gap-2">
            <Button size="sm" variant="outline" on:click={() => test(channel)}>Test</Button>
            <Button size="sm" variant="outline" on:click={() => edit(channel)}>Edit</Button>
            <Button
              size="sm"
              variant="ghost"
              class="ml-auto text-destructive"
              on:click={() => (deleting = channel)}>Delete</Button
            >
          </Card.Footer>
        {/if}
      </Card.Root>
    {/each}
  </div>
{/if}

<Card.Root>
  <Card.Header>
    <Card.Title>History</Card.Title>
    <Card.Description>The latest alerts and whether they were delivered.</Card.Description>
  </Card.Header>
  <Card.Content>
    {#if history.length === 0}
      <p class="text-sm text-muted-foreground">No alerts sent yet.</p>
    {:else}
      <Table.Root>
        <Table.Header>
          <Table.Row>
            <Table.Head>Time</Table.Head>
            <Table.Head>Alert</Table.Head>
            <Table.Head>Channel</Table.Head>
            <Table.Head>Delivery</Table.Head>
          </Table.Row>
        </Table.Header>
        <Table.Body>
          {#each history as delivery (delivery.id)}
            <Table.Row>
              <Table.Cell class="whitespace-nowrap text-muted-foreground"
                >{formatTime(delivery.sent_at)}</Table.Cell
              >
              <Table.Cell>
                <div class="font-medium">{delivery.title}</div>
                {#if delivery.message}
                  <div class="text-xs text-muted-foreground">{delivery.message}</div>
                {/if}
              </Table.Cell>
              <Table.Cell>{delivery.channel_name ?? 'Deleted channel'}</Table.Cell>
              <Table.Cell>
                {#if delivery.status === 'sent'}
                  <Badge>Sent</Badge>
                {:else}
                  <Badge variant="destructive">Failed</Badge>
                  {#if delivery.error}
                    <div class="mt-1 max-w-xs text-xs text-muted-foreground">{delivery.error}</div>
                  {/if}
                {/if}
              </Table.Cell>
            </Table.Row>
          {/each}
        </Table.Body>
      </Table.Root>
    {/if}
  </Card.Content>
</Card.Root>

<ChannelForm bind:open={formOpen} channel={editing} on:saved={load} />

<Dialog.Root open={deleting !== null} onOpenChange={(open) => !open && (deleting = null)}>
  <Dialog.Content class="sm:max-w-[425px]">
    <Dialog.Header>
      <Dialog.Title>Delete channel</Dialog.Title>
      <Dialog.Description>
        Delete "{deleting?.name}"? Services stop sending alerts to it. Its history is kept.
      </Dialog.Description>
    </Dialog.Header>
    <Dialog.Footer>
      <Button variant="outline" on:click={() => (deleting = null)}>Cancel</Button>
      <Button variant="destructive" on:click={confirmDelete}>Yes, delete</Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>
