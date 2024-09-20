<script>
  import { toast } from 'svelte-sonner';
  import { cfetch } from '$lib/utils.js';

  import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
  import { Button } from '$lib/components/ui/button/index.js';
  import { CirclePlus, File, ListFilter } from 'lucide-svelte';
  import * as Dialog from '$lib/components/ui/dialog/index.js';
  import { Input } from '$lib/components/ui/input/index.js';
  import { Label } from '$lib/components/ui/label/index.js';
  import * as Select from '$lib/components/ui/select/index.js';

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
   * @property {number} retry_interval - The number of retries.
   * @property {number} interval - The number of retries.
   * @property {number} timeout - The retry interval in seconds.
   */

  /** @type {NewService} */
  let newService = {
    name: '',
    url: '',
    service_type: '',
    retry: 1,
    retry_interval: 30,
    interval: 60,
    timeout: 20
  };

  /** @type {import('../$types').Snapshot<NewService>} */
  export const snapshot = {
    capture: () => newService,
    restore: (value) => (newService = value)
  };

  function addService() {
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

<div class="flex items-center">
  <DropdownMenu.Root>
    <DropdownMenu.Trigger asChild let:builder>
      <Button builders={[builder]} variant="outline" size="sm" class="h-8 gap-1">
        <ListFilter class="h-3.5 w-3.5" />
        <span class="sr-only sm:not-sr-only sm:whitespace-nowrap"> Filter </span>
      </Button>
    </DropdownMenu.Trigger>
    <DropdownMenu.Content align="end">
      <DropdownMenu.Label>Filter by</DropdownMenu.Label>
      <DropdownMenu.Separator />
      <DropdownMenu.CheckboxItem checked>Active</DropdownMenu.CheckboxItem>
      <DropdownMenu.CheckboxItem>Inactive</DropdownMenu.CheckboxItem>
      <DropdownMenu.CheckboxItem>Down</DropdownMenu.CheckboxItem>
    </DropdownMenu.Content>
  </DropdownMenu.Root>
  <div class="ml-auto flex items-center gap-2">
    <Button size="sm" variant="outline" class="h-8 gap-1">
      <File class="h-3.5 w-3.5" />
      <span class="sr-only sm:not-sr-only sm:whitespace-nowrap"> Export </span>
    </Button>
    <Dialog.Root>
      <Dialog.Trigger asChild let:builder>
        <Button builders={[builder]} size="sm" class="h-8 gap-1">
          <CirclePlus class="h-3.5 w-3.5" />
          <span class="sr-only sm:not-sr-only sm:whitespace-nowrap"> Add Service </span>
        </Button>
      </Dialog.Trigger>
      <Dialog.Content class="sm:max-w-[425px]">
        <Dialog.Header>
          <Dialog.Title>Add Service</Dialog.Title>
          <Dialog.Description>Add a new service to monitor</Dialog.Description>
        </Dialog.Header>
        <form on:submit|preventDefault={addService}>
          <div class="grid gap-4 py-4">
            <div class="grid grid-cols-4 items-center gap-4">
              <Label for="name" class="text-right">Name</Label>
              <Input
                bind:value={newService.name}
                id="name"
                placeholder="Service Name"
                class="col-span-3"
                required
              />
            </div>
            <div class="grid grid-cols-4 items-center gap-4">
              <Label for="url" class="text-right">Url</Label>
              <Input
                id="url"
                bind:value={newService.url}
                placeholder="URL/Ip of the service"
                class="col-span-3"
                required
              />
            </div>
            <div class="grid grid-cols-4 items-center gap-4">
              <Label for="type" class="text-right">Type</Label>
              <Select.Root
                onSelectedChange={(v) => (newService.service_type = v?.value)}
                portal={null}
              >
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
            <div class="grid grid-cols-4 items-center gap-4">
              <Label for="interval" class="text-right">Interval</Label>
              <Input
                id="interval"
                bind:value={newService.interval}
                placeholder="Monitor interval"
                type="number"
                class="col-span-3"
              />
            </div>
            <div class="grid grid-cols-4 items-center gap-4">
              <Label for="timeout" class="text-right">Timeout</Label>
              <Input
                id="timeout"
                placeholder="Timeout in seconds"
                bind:value={newService.timeout}
                type="number"
                class="col-span-3"
              />
            </div>
          </div>
          <Dialog.Footer>
            <Button type="submit">Add Service</Button>
          </Dialog.Footer>
        </form>
      </Dialog.Content>
    </Dialog.Root>
  </div>
</div>
