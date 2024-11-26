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

  /**
   * @typedef {Object} NewService
   * @property {string} name - The monitor name.
   * @property {string} url - The monitor URL.
   * @property {string} service_type - The monitor type.
   * @property {number} retry - The number of retries.
   * @property {number} retry_interval - The interval between retries in seconds.
   * @property {number} interval - The monitoring interval in seconds.
   * @property {number} timeout - The request timeout in seconds.
   * @property {boolean} invert - Invert the expected result.
   * @property {number} expected_code - The expected HTTP status code.
   * @property {string} expected_payload - The expected HTTP response body.
   */

  /** @type {NewService} */
  let newService = {
    name: '',
    url: '',
    service_type: '',
    retry: 1,
    retry_interval: 30,
    interval: 60,
    timeout: 20,
    invert: false,
    expected_code: 2,
    expected_payload: ''
  };

  /** @type {import('../$types').Snapshot<NewService>} */
  export const snapshot = {
    capture: () => newService,
    restore: (value) => (newService = value)
  };

  function addService() {
    // Validate inputs
    if (newService.timeout >= newService.interval) {
      toast.error('Timeout must be less than interval');
      return;
    }

    // fix data types
    newService.retry = Number(newService.retry);
    newService.retry_interval = Number(newService.retry_interval);
    newService.interval = Number(newService.interval);
    newService.timeout = Number(newService.timeout);
    newService.expected_code = Number(newService.expected_code);
    console.log(newService);
    const promise = new Promise((resolve, reject) =>
      cfetch('/services', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        credentials: 'same-origin',
        body: JSON.stringify(newService)
      })
        .then(async (res) => {
          if (res.ok) {
            const data = await res.json();
            // console.log(data);
            resolve(data);
          } else {
            console.log(res);
            reject(res.statusText);
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
      <Label for="url" class="sm:text-right">Url</Label>
      <Input
        id="url"
        bind:value={newService.url}
        placeholder="URL/Ip of the service"
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
    <div class="grid grid-cols-1 items-center gap-4 sm:grid-cols-4">
      <Label for="payload" class="sm:text-right">Payload</Label>
      <Textarea id="payload" placeholder="Request payload" class="col-span-3" />
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
    <div class="grid grid-cols-1 items-center gap-4 sm:grid-cols-4">
      <Label for="expected_code" class="sm:text-right">Expected Code</Label>
      <Input
        id="expected_code"
        placeholder="Expected HTTP response code"
        bind:value={newService.expected_code}
        type="number"
        class="col-span-3"
      />
    </div>
    <div class="grid grid-cols-1 items-center gap-4 sm:grid-cols-4">
      <Label for="expected_payload" class="sm:text-right">Expected Payload</Label>
      <Textarea
        id="expected_payload"
        placeholder="Expected response payload"
        bind:value={newService.expected_payload}
        class="col-span-3"
      />
    </div>
  </div>
  <Footer class="gap-2">
    <Button
      variant="outline"
      type="reset"
      on:click={() => {
        newService = {
          name: '',
          url: '',
          service_type: '',
          retry: 1,
          retry_interval: 30,
          interval: 60,
          timeout: 20,
          invert: false,
          expected_code: 200,
          expected_payload: ''
        };
      }}>Reset</Button
    >
    <Button type="submit">Add Service</Button>
  </Footer>
</form>
