<script context="module">
  /**
   * @typedef {Object} EditableChannel
   * @property {number} id
   * @property {string} name
   * @property {boolean} apply_to_all
   * @property {Record<string, any>} config
   */
</script>

<script>
  import { createEventDispatcher } from 'svelte';
  import { toast } from 'svelte-sonner';

  import { Button } from '$lib/components/ui/button/index.js';
  import * as Dialog from '$lib/components/ui/dialog/index.js';
  import { Input } from '$lib/components/ui/input/index.js';
  import { Label } from '$lib/components/ui/label/index.js';
  import * as Select from '$lib/components/ui/select/index.js';
  import { Switch } from '$lib/components/ui/switch/index.js';
  import { Textarea } from '$lib/components/ui/textarea/index.js';
  import { cfetch } from '$lib/utils.js';

  export let open = false;
  /**
   * The channel to edit, with its config; `null` to add a new one.
   * @type {EditableChannel | null}
   */
  export let channel = null;

  const dispatch = createEventDispatcher();

  const channelTypes = [
    { value: 'webhook', label: 'Webhook' },
    { value: 'slack', label: 'Slack' },
    { value: 'discord', label: 'Discord' },
    { value: 'telegram', label: 'Telegram' },
    { value: 'ntfy', label: 'ntfy' },
    { value: 'email', label: 'Email (SMTP)' }
  ];
  const securityOptions = [
    { value: 'starttls', label: 'STARTTLS' },
    { value: 'tls', label: 'TLS' },
    { value: 'none', label: 'None' }
  ];

  /** Flat form state covering every channel type. */
  /** @param {EditableChannel | null} channel */
  function formFrom(channel) {
    const config = channel?.config ?? {};
    return {
      name: channel?.name ?? '',
      apply_to_all: channel?.apply_to_all ?? false,
      type: config.type ?? '',
      url: config.url ?? '',
      headers: Object.entries(config.headers ?? {})
        .map(([name, value]) => `${name}: ${value}`)
        .join('\n'),
      webhook_url: config.webhook_url ?? '',
      bot_token: config.bot_token ?? '',
      chat_id: config.chat_id ?? '',
      server_url: config.server_url ?? 'https://ntfy.sh',
      topic: config.topic ?? '',
      access_token: config.access_token ?? '',
      host: config.host ?? '',
      port: config.port ?? 587,
      security: config.security ?? 'starttls',
      username: config.username ?? '',
      password: config.password ?? '',
      from: config.from ?? '',
      to: (config.to ?? []).join('\n')
    };
  }

  let form = formFrom(channel);
  $: if (open) form = formFrom(channel);

  /**
   * @param {string} text
   * @returns {Record<string, string>}
   */
  function parseHeaders(text) {
    /** @type {Record<string, string>} */
    const headers = {};
    for (const line of text.split('\n')) {
      if (!line.trim()) continue;
      const split = line.indexOf(':');
      if (split <= 0) throw new Error(`Invalid header line: "${line}"`);
      headers[line.slice(0, split).trim()] = line.slice(split + 1).trim();
    }
    return headers;
  }

  function buildConfig() {
    switch (form.type) {
      case 'webhook':
        return { type: 'webhook', url: form.url, headers: parseHeaders(form.headers) };
      case 'slack':
      case 'discord':
        return { type: form.type, webhook_url: form.webhook_url };
      case 'telegram':
        return { type: 'telegram', bot_token: form.bot_token, chat_id: form.chat_id };
      case 'ntfy':
        return {
          type: 'ntfy',
          server_url: form.server_url,
          topic: form.topic,
          access_token: form.access_token || null
        };
      case 'email':
        return {
          type: 'email',
          host: form.host,
          port: Number(form.port),
          security: form.security,
          username: form.username || null,
          password: form.password || null,
          from: form.from,
          to: form.to
            .split(/[\n,]/)
            .map((/** @type {string} */ v) => v.trim())
            .filter(Boolean)
        };
      default:
        throw new Error('Select a channel type');
    }
  }

  async function save() {
    let config;
    try {
      config = buildConfig();
    } catch (e) {
      toast.error(`${e instanceof Error ? e.message : e}`);
      return;
    }
    const body = { name: form.name, apply_to_all: form.apply_to_all, config };
    const res = await cfetch(channel ? `/channels/${channel.id}` : '/channels', {
      method: channel ? 'PUT' : 'POST',
      headers: { 'Content-Type': 'application/json' },
      credentials: 'same-origin',
      body: JSON.stringify(body)
    });
    const data = await res.json().catch(() => null);
    if (!res.ok) {
      toast.error(`Error: ${data?.message ?? data?.error ?? res.statusText}`);
      return;
    }
    toast.success(channel ? `Channel "${form.name}" updated` : `Channel "${form.name}" added`);
    open = false;
    dispatch('saved');
  }
</script>

<Dialog.Root bind:open>
  <Dialog.Content class="max-h-[90vh] overflow-y-auto sm:max-w-[520px]">
    <Dialog.Header>
      <Dialog.Title>{channel ? 'Edit channel' : 'Add channel'}</Dialog.Title>
      <Dialog.Description
        >Where Stamon sends alerts when a service goes down or recovers.</Dialog.Description
      >
    </Dialog.Header>
    <form id="channel-form" class="grid gap-4 py-2" on:submit|preventDefault={save}>
      <div class="grid gap-2">
        <Label for="channel-name">Name</Label>
        <Input id="channel-name" bind:value={form.name} placeholder="e.g. Ops Slack" required />
      </div>
      <div class="grid gap-2">
        <Label for="channel-type">Type</Label>
        <Select.Root
          selected={channelTypes.find((t) => t.value === form.type)}
          onSelectedChange={(/** @type {{ value: string } | undefined} */ v) =>
            (form.type = v?.value ?? '')}
          portal={null}
        >
          <Select.Trigger class="w-[200px]">
            <Select.Value placeholder="Select a type" />
          </Select.Trigger>
          <Select.Content>
            {#each channelTypes as type}
              <Select.Item value={type.value} label={type.label}>{type.label}</Select.Item>
            {/each}
          </Select.Content>
          <Select.Input id="channel-type" required />
        </Select.Root>
      </div>

      {#if form.type === 'webhook'}
        <div class="grid gap-2">
          <Label for="url">URL</Label>
          <Input id="url" type="url" bind:value={form.url} placeholder="https://…" required />
          <p class="text-xs text-muted-foreground">
            Receives a JSON POST with the service, status, message, title and text.
          </p>
        </div>
        <div class="grid gap-2">
          <Label for="headers">Headers</Label>
          <Textarea
            id="headers"
            bind:value={form.headers}
            placeholder="Authorization: Bearer <token>"
            class="font-mono"
          />
        </div>
      {:else if form.type === 'slack' || form.type === 'discord'}
        <div class="grid gap-2">
          <Label for="webhook_url">Webhook URL</Label>
          <Input
            id="webhook_url"
            type="url"
            bind:value={form.webhook_url}
            placeholder={form.type === 'slack'
              ? 'https://hooks.slack.com/services/…'
              : 'https://discord.com/api/webhooks/…'}
            required
          />
        </div>
      {:else if form.type === 'telegram'}
        <div class="grid gap-2">
          <Label for="bot_token">Bot Token</Label>
          <Input
            id="bot_token"
            bind:value={form.bot_token}
            placeholder="123456:ABC-DEF…"
            required
          />
        </div>
        <div class="grid gap-2">
          <Label for="chat_id">Chat ID</Label>
          <Input
            id="chat_id"
            bind:value={form.chat_id}
            placeholder="e.g. -1001234567890"
            required
          />
        </div>
      {:else if form.type === 'ntfy'}
        <div class="grid gap-2">
          <Label for="server_url">Server</Label>
          <Input id="server_url" type="url" bind:value={form.server_url} required />
        </div>
        <div class="grid gap-2">
          <Label for="topic">Topic</Label>
          <Input id="topic" bind:value={form.topic} placeholder="stamon-alerts" required />
        </div>
        <div class="grid gap-2">
          <Label for="access_token">Access Token</Label>
          <Input
            id="access_token"
            type="password"
            bind:value={form.access_token}
            placeholder="Only for protected topics"
          />
        </div>
      {:else if form.type === 'email'}
        <div class="grid grid-cols-3 gap-2">
          <div class="col-span-2 grid gap-2">
            <Label for="smtp_host">SMTP Host</Label>
            <Input id="smtp_host" bind:value={form.host} placeholder="smtp.example.com" required />
          </div>
          <div class="grid gap-2">
            <Label for="smtp_port">Port</Label>
            <Input id="smtp_port" type="number" min="1" max="65535" bind:value={form.port} />
          </div>
        </div>
        <div class="grid gap-2">
          <Label for="security">Security</Label>
          <Select.Root
            selected={securityOptions.find((o) => o.value === form.security)}
            onSelectedChange={(/** @type {{ value: string } | undefined} */ v) =>
              (form.security = v?.value ?? 'starttls')}
            portal={null}
          >
            <Select.Trigger class="w-[200px]">
              <Select.Value placeholder="STARTTLS" />
            </Select.Trigger>
            <Select.Content>
              {#each securityOptions as option}
                <Select.Item value={option.value} label={option.label}>{option.label}</Select.Item>
              {/each}
            </Select.Content>
            <Select.Input id="security" />
          </Select.Root>
        </div>
        <div class="grid grid-cols-2 gap-2">
          <div class="grid gap-2">
            <Label for="smtp_username">Username</Label>
            <Input id="smtp_username" bind:value={form.username} autocomplete="off" />
          </div>
          <div class="grid gap-2">
            <Label for="smtp_password">Password</Label>
            <Input
              id="smtp_password"
              type="password"
              bind:value={form.password}
              autocomplete="new-password"
            />
          </div>
        </div>
        <div class="grid gap-2">
          <Label for="from">From</Label>
          <Input
            id="from"
            bind:value={form.from}
            placeholder="Stamon <stamon@example.com>"
            required
          />
        </div>
        <div class="grid gap-2">
          <Label for="to">To</Label>
          <Textarea
            id="to"
            bind:value={form.to}
            placeholder="One address per line"
            class="font-mono"
            required
          />
        </div>
      {/if}

      <div class="flex items-center justify-between gap-4">
        <div>
          <Label for="apply_to_all">All services</Label>
          <p class="text-xs text-muted-foreground">
            Send alerts for every service, not only the ones that pick this channel.
          </p>
        </div>
        <Switch id="apply_to_all" bind:checked={form.apply_to_all} />
      </div>
    </form>
    <Dialog.Footer>
      <Button variant="outline" on:click={() => (open = false)}>Cancel</Button>
      <Button type="submit" form="channel-form">{channel ? 'Save' : 'Add channel'}</Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>
