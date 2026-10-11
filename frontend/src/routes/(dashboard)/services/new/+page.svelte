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
  import { cfetch } from '$lib/utils.js';

  const serviceTypes = [
    { value: 'ping', label: 'Ping' },
    { value: 'http', label: 'HTTP(s)' }
  ];

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
   * @property {string} host - Ping: the IP address or hostname.
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
      host: ''
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
          expected_payload: newService.expected_payload || null
        };
      case 'ping':
        return { type: 'ping', host: newService.host };
      default:
        throw new Error('Select a service type');
    }
  }

  function addService() {
    // Validate inputs
    if (Number(newService.timeout) >= Number(newService.interval)) {
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
      config
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
    {:else if newService.service_type === 'ping'}
      <div class="grid grid-cols-1 items-center gap-4 sm:grid-cols-4">
        <Label for="host" class="sm:text-right">Host</Label>
        <Input
          id="host"
          bind:value={newService.host}
          placeholder="IP address or hostname"
          class="col-span-3"
          required
        />
      </div>
    {/if}

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
  </div>
  <Footer class="gap-2">
    <Button variant="outline" type="reset" on:click={() => (newService = emptyService())}
      >Reset</Button
    >
    <Button type="submit">Add Service</Button>
  </Footer>
</form>
