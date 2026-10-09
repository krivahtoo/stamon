<script>
  import { fly } from 'svelte/transition';
  import { toast } from 'svelte-sonner';
  import ChevronLeft from 'lucide-svelte/icons/chevron-left';

  import { Button } from '$lib/components/ui/button/index.js';
  import { Footer } from '$lib/components/ui/dialog/index.js';
  import { Input } from '$lib/components/ui/input/index.js';
  import { Label } from '$lib/components/ui/label/index.js';
  import * as Select from '$lib/components/ui/select/index.js';
  import { Switch } from '$lib/components/ui/switch/index.js';
  import { Textarea } from '$lib/components/ui/textarea/index.js';
  import { Separator } from '$lib/components/ui/separator/index.js';
  import ChannelPicker from '$lib/components/channel-picker.svelte';
  import { cfetch, pushUrl } from '$lib/utils.js';

  const serviceTypes = [
    { value: 'http', label: 'HTTP(s)' },
    { value: 'ping', label: 'Ping' },
    { value: 'tcp', label: 'TCP Port' },
    { value: 'dns', label: 'DNS' },
    { value: 'tls', label: 'TLS Certificate' },
    { value: 'push', label: 'Push (heartbeat)' }
  ];

  const recordTypes = ['A', 'AAAA', 'CNAME', 'MX', 'NS', 'TXT'].map((t) => ({
    value: t,
    label: t
  }));

  /** A random token for a push URL; works without a secure context. */
  function newPushToken() {
    const bytes = crypto.getRandomValues(new Uint8Array(16));
    return Array.from(bytes, (b) => b.toString(16).padStart(2, '0')).join('');
  }

  const httpMethods = ['GET', 'HEAD', 'POST', 'PUT', 'PATCH', 'DELETE', 'OPTIONS'].map((m) => ({
    value: m,
    label: m
  }));

  /**
   * @typedef {Object} NewService
   * @property {string} name - The monitor name.
   * @property {string} service_type - The monitor type.
   * @property {number} retry - The number of retries.
   * @property {number} retry_interval - The interval between retries in seconds.
   * @property {number} interval - The monitoring interval in seconds.
   * @property {number} timeout - The check timeout in seconds.
   * @property {boolean} invert - Invert the expected result.
   * @property {string} url - HTTP: the URL to request.
   * @property {string} method - HTTP: the request method.
   * @property {string} headers - HTTP: request headers, one `Name: value` per line.
   * @property {string} body - HTTP: the request body.
   * @property {number | string} expected_code - HTTP: expected status code, or 1-5 for a class.
   * @property {string} expected_payload - HTTP: JSON the response must equal.
   * @property {string} keyword - HTTP: text the response must contain.
   * @property {string} json_pointer - HTTP: JSON pointer to a value in the response.
   * @property {string} json_expected - HTTP: what the value at json_pointer must be.
   * @property {string} host - Ping, TCP, DNS, TLS: the IP address or hostname.
   * @property {number | string} port - TCP, TLS: the port.
   * @property {string} record_type - DNS: the record type to look up.
   * @property {string} resolver - DNS: nameserver to ask, `ip` or `ip:port`.
   * @property {string} expected_records - DNS: values that must be answered, one per line.
   * @property {number | string} warn_days - TLS: days before expiry to report down.
   * @property {string} push_token - Push: the secret in the push URL.
   * @property {number | string} grace_secs - Push: extra seconds before a heartbeat is late.
   * @property {number[]} channel_ids - Channels that receive this service's alerts.
   * @property {string} tags - Comma-separated labels.
   */

  /** @returns {NewService} */
  function emptyService() {
    return {
      name: '',
      service_type: '',
      retry: 1,
      retry_interval: 30,
      interval: 60,
      timeout: 20,
      invert: false,
      url: '',
      method: 'GET',
      headers: '',
      body: '',
      expected_code: 2,
      expected_payload: '',
      keyword: '',
      json_pointer: '',
      json_expected: '',
      host: '',
      port: '',
      record_type: 'A',
      resolver: '',
      expected_records: '',
      warn_days: 14,
      push_token: newPushToken(),
      grace_secs: 0,
      channel_ids: [],
      tags: ''
    };
  }

  /** @type {NewService} */
  let newService = emptyService();

  /** @type {import('../$types').Snapshot<NewService>} */
  export const snapshot = {
    capture: () => newService,
    restore: (value) => (newService = value)
  };

  /**
   * Parse `Name: value` lines into a header map.
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

  /** Build the type-specific check config sent to the API. */
  function buildConfig() {
    switch (newService.service_type) {
      case 'http':
        return {
          type: 'http',
          url: newService.url,
          method: newService.method,
          headers: parseHeaders(newService.headers),
          body: newService.body || null,
          expected_code: newService.expected_code === '' ? null : Number(newService.expected_code),
          expected_payload: newService.expected_payload || null,
          keyword: newService.keyword || null,
          json_pointer: newService.json_pointer || null,
          json_expected: newService.json_expected || null
        };
      case 'ping':
        return { type: 'ping', host: newService.host };
      case 'tcp':
        return { type: 'tcp', host: newService.host, port: Number(newService.port) };
      case 'dns':
        return {
          type: 'dns',
          host: newService.host,
          record_type: newService.record_type,
          resolver: newService.resolver || null,
          expected: newService.expected_records
            .split('\n')
            .map((v) => v.trim())
            .filter(Boolean)
        };
      case 'tls':
        return {
          type: 'tls',
          host: newService.host,
          port: Number(newService.port) || 443,
          warn_days: Number(newService.warn_days)
        };
      case 'push':
        return {
          type: 'push',
          token: newService.push_token,
          grace_secs: Number(newService.grace_secs) || 0
        };
      default:
        throw new Error('Select a service type');
    }
  }

  function addService() {
    // Validate inputs; push monitors aren't checked, so they have no timeout.
    if (
      newService.service_type !== 'push' &&
      Number(newService.timeout) >= Number(newService.interval)
    ) {
      toast.error('Timeout must be less than interval');
      return;
    }

    let config;
    try {
      config = buildConfig();
    } catch (e) {
      toast.error(`${e instanceof Error ? e.message : e}`);
      return;
    }

    const payload = {
      name: newService.name,
      retry: Number(newService.retry),
      retry_interval: Number(newService.retry_interval),
      interval: Number(newService.interval),
      timeout: Number(newService.timeout),
      invert: newService.invert,
      config,
      tags: newService.tags.split(','),
      channel_ids: newService.channel_ids
    };
    const promise = new Promise((resolve, reject) =>
      cfetch('/services', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        credentials: 'same-origin',
        body: JSON.stringify(payload)
      })
        .then(async (res) => {
          if (res.ok) {
            resolve(await res.json());
          } else {
            const data = await res.json().catch(() => null);
            reject(data?.message ?? data?.error ?? res.statusText);
          }
        })
        .catch((e) => {
          reject(e);
        })
    );

    toast.promise(promise, {
      loading: 'Loading...',
      success: `Monitor "${newService.name}" added`,
      error: (e) => {
        return `Error: ${e}`;
      }
    });
  }
</script>

<div in:fly={{ y: 20, duration: 200 }} class="flex items-center justify-start space-y-2">
  <Button
    variant="ghost"
    size="icon"
    class="h-7 w-7 md:mx-6 lg:mx-10 lg:h-auto lg:w-auto lg:p-2"
    on:click={() => history.back()}
  >
    <ChevronLeft class="h-4 w-4" />
    <span class="sr-only lg:not-sr-only">Back</span>
  </Button>
  <div>
    <h2 class="text-2xl font-bold tracking-tight">New Service</h2>
    <p class="text-muted-foreground">Add a new service to monitor!</p>
  </div>
</div>

<Separator class="my-1" />

<form on:submit|preventDefault={addService}>
  <div class="grid gap-4 py-4">
    <div class="grid grid-cols-1 items-center gap-4 sm:grid-cols-4">
      <Label for="name" class="sm:text-right">Name</Label>
      <Input
        bind:value={newService.name}
        id="name"
        placeholder="Service Name"
        class="col-span-3"
        required
      />
    </div>
    <div class="grid grid-cols-1 items-center gap-4 sm:grid-cols-4">
      <Label for="type" class="sm:text-right">Type</Label>
      <Select.Root onSelectedChange={(v) => (newService.service_type = v?.value)} portal={null}>
        <Select.Trigger class="w-[180px]">
          <Select.Value placeholder="Select a service type" />
        </Select.Trigger>
        <Select.Content>
          <Select.Group>
            <Select.Label>Basic Types</Select.Label>
            {#each serviceTypes as type}
              <Select.Item value={type.value} label={type.label}>{type.label}</Select.Item>
            {/each}
          </Select.Group>
        </Select.Content>
        <Select.Input name="service_type" id="type" required />
      </Select.Root>
    </div>

    {#if newService.service_type === 'http'}
      <div class="grid grid-cols-1 items-center gap-4 sm:grid-cols-4">
        <Label for="url" class="sm:text-right">URL</Label>
        <Input
          id="url"
          bind:value={newService.url}
          placeholder="https://example.com/health"
          type="url"
          class="col-span-3"
          required
        />
      </div>
      <div class="grid grid-cols-1 items-center gap-4 sm:grid-cols-4">
        <Label for="method" class="sm:text-right">Method</Label>
        <Select.Root
          selected={{ value: newService.method, label: newService.method }}
          onSelectedChange={(v) => (newService.method = v?.value ?? 'GET')}
          portal={null}
        >
          <Select.Trigger class="w-[180px]">
            <Select.Value placeholder="GET" />
          </Select.Trigger>
          <Select.Content>
            {#each httpMethods as method}
              <Select.Item value={method.value} label={method.label}>{method.label}</Select.Item>
            {/each}
          </Select.Content>
          <Select.Input name="method" id="method" />
        </Select.Root>
      </div>
      <div class="grid grid-cols-1 items-center gap-4 sm:grid-cols-4">
        <Label for="headers" class="sm:text-right">Headers</Label>
        <Textarea
          id="headers"
          bind:value={newService.headers}
          placeholder={'Authorization: Bearer <token>\nAccept: application/json'}
          class="col-span-3 font-mono"
        />
      </div>
      <div class="grid grid-cols-1 items-center gap-4 sm:grid-cols-4">
        <Label for="body" class="sm:text-right">Body</Label>
        <Textarea
          id="body"
          bind:value={newService.body}
          placeholder="Request body"
          class="col-span-3 font-mono"
        />
      </div>
      <div class="grid grid-cols-1 items-center gap-4 sm:grid-cols-4">
        <Label for="expected_code" class="sm:text-right">Expected Code</Label>
        <Input
          id="expected_code"
          placeholder="Status code, or 1-5 for a whole class (2 = any 2xx)"
          bind:value={newService.expected_code}
          type="number"
          min="0"
          max="599"
          class="col-span-3"
        />
      </div>
      <div class="grid grid-cols-1 items-center gap-4 sm:grid-cols-4">
        <Label for="expected_payload" class="sm:text-right">Expected Payload</Label>
        <Textarea
          id="expected_payload"
          placeholder="JSON the response must equal"
          bind:value={newService.expected_payload}
          class="col-span-3 font-mono"
        />
      </div>
      <div class="grid grid-cols-1 items-center gap-4 sm:grid-cols-4">
        <Label for="keyword" class="sm:text-right">Keyword</Label>
        <Input
          id="keyword"
          bind:value={newService.keyword}
          placeholder="Text the response must contain"
          class="col-span-3"
        />
      </div>
      <div class="grid grid-cols-1 items-center gap-4 sm:grid-cols-4">
        <Label for="json_pointer" class="sm:text-right">JSON Value</Label>
        <div class="col-span-3 grid grid-cols-1 gap-2 sm:grid-cols-2">
          <Input
            id="json_pointer"
            bind:value={newService.json_pointer}
            placeholder="Pointer, e.g. /status"
            class="font-mono"
          />
          <Input
            id="json_expected"
            bind:value={newService.json_expected}
            placeholder="Must equal, e.g. ok (optional)"
            class="font-mono"
          />
        </div>
      </div>
    {:else if ['ping', 'tcp', 'dns', 'tls'].includes(newService.service_type)}
      <div class="grid grid-cols-1 items-center gap-4 sm:grid-cols-4">
        <Label for="host" class="sm:text-right">Host</Label>
        <Input
          id="host"
          bind:value={newService.host}
          placeholder={newService.service_type === 'dns'
            ? 'Name to look up, e.g. example.com'
            : 'IP address or hostname'}
          class="col-span-3"
          required
        />
      </div>
      {#if newService.service_type === 'tcp' || newService.service_type === 'tls'}
        <div class="grid grid-cols-1 items-center gap-4 sm:grid-cols-4">
          <Label for="port" class="sm:text-right">Port</Label>
          <Input
            id="port"
            bind:value={newService.port}
            placeholder={newService.service_type === 'tls' ? '443' : 'e.g. 5432'}
            type="number"
            min="1"
            max="65535"
            class="col-span-3"
            required={newService.service_type === 'tcp'}
          />
        </div>
      {/if}
      {#if newService.service_type === 'tls'}
        <div class="grid grid-cols-1 items-center gap-4 sm:grid-cols-4">
          <Label for="warn_days" class="sm:text-right">Warn Days</Label>
          <Input
            id="warn_days"
            bind:value={newService.warn_days}
            placeholder="Report down this many days before expiry"
            type="number"
            min="0"
            class="col-span-3"
          />
        </div>
      {/if}
      {#if newService.service_type === 'dns'}
        <div class="grid grid-cols-1 items-center gap-4 sm:grid-cols-4">
          <Label for="record_type" class="sm:text-right">Record Type</Label>
          <Select.Root
            selected={{ value: newService.record_type, label: newService.record_type }}
            onSelectedChange={(v) => (newService.record_type = v?.value ?? 'A')}
            portal={null}
          >
            <Select.Trigger class="w-[180px]">
              <Select.Value placeholder="A" />
            </Select.Trigger>
            <Select.Content>
              {#each recordTypes as recordType}
                <Select.Item value={recordType.value} label={recordType.label}
                  >{recordType.label}</Select.Item
                >
              {/each}
            </Select.Content>
            <Select.Input name="record_type" id="record_type" />
          </Select.Root>
        </div>
        <div class="grid grid-cols-1 items-center gap-4 sm:grid-cols-4">
          <Label for="resolver" class="sm:text-right">Resolver</Label>
          <Input
            id="resolver"
            bind:value={newService.resolver}
            placeholder="Nameserver IP, e.g. 1.1.1.1 (default: system)"
            class="col-span-3"
          />
        </div>
        <div class="grid grid-cols-1 items-center gap-4 sm:grid-cols-4">
          <Label for="expected_records" class="sm:text-right">Expected Values</Label>
          <Textarea
            id="expected_records"
            bind:value={newService.expected_records}
            placeholder={'Values that must be answered, one per line\ne.g. 93.184.215.14'}
            class="col-span-3 font-mono"
          />
        </div>
      {/if}
    {:else if newService.service_type === 'push'}
      <div class="grid grid-cols-1 items-center gap-4 sm:grid-cols-4">
        <Label for="push_url" class="sm:text-right">Push URL</Label>
        <div class="col-span-3 space-y-1">
          <Input id="push_url" readonly value={pushUrl(newService.push_token)} class="font-mono" />
          <p class="text-xs text-muted-foreground">
            Have your service call this URL at least once per interval. Add
            <code>?status=down&amp;msg=...</code> to report a failure.
          </p>
        </div>
      </div>
      <div class="grid grid-cols-1 items-center gap-4 sm:grid-cols-4">
        <Label for="grace_secs" class="sm:text-right">Grace Period</Label>
        <Input
          id="grace_secs"
          bind:value={newService.grace_secs}
          placeholder="Extra seconds before a heartbeat counts as missed"
          type="number"
          min="0"
          class="col-span-3"
        />
      </div>
    {/if}

    <div class="grid grid-cols-1 items-center gap-4 sm:grid-cols-4">
      <Label for="tags" class="sm:text-right">Tags</Label>
      <Input
        id="tags"
        bind:value={newService.tags}
        placeholder="Comma-separated, e.g. prod, api"
        class="col-span-3"
      />
    </div>
    <div class="grid grid-cols-1 items-center gap-4 sm:grid-cols-4">
      <Label for="interval" class="sm:text-right">Interval</Label>
      <Input
        id="interval"
        bind:value={newService.interval}
        placeholder="Monitor interval"
        type="number"
        min="30"
        max="86400"
        class="col-span-3"
      />
    </div>
    {#if newService.service_type !== 'push'}
      <div class="grid grid-cols-1 items-center gap-4 sm:grid-cols-4">
        <Label for="timeout" class="sm:text-right">Timeout</Label>
        <Input
          id="timeout"
          placeholder="Timeout in seconds"
          bind:value={newService.timeout}
          min="1"
          max="60"
          type="number"
          class="col-span-3"
        />
      </div>
      <div class="grid grid-cols-4 items-center gap-4">
        <Label for="invert" class="sm:text-right">Invert Check</Label>
        <Switch id="invert" bind:checked={newService.invert} class="sm:col-span-3" />
      </div>
    {/if}
    <div class="grid grid-cols-1 items-start gap-4 sm:grid-cols-4">
      <Label class="sm:pt-1 sm:text-right">Alert Channels</Label>
      <div class="col-span-3">
        <ChannelPicker bind:selected={newService.channel_ids} />
      </div>
    </div>
  </div>
  <Footer class="gap-2">
    <Button variant="outline" type="reset" on:click={() => (newService = emptyService())}
      >Reset</Button
    >
    <Button type="submit">Add Service</Button>
  </Footer>
</form>
