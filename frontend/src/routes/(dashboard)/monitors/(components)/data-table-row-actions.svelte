<script>
  import { Button } from '$lib/components/ui/button/index.js';
  import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
  import * as Dialog from '$lib/components/ui/dialog/index.js';

  import { EllipsisVertical } from 'lucide-svelte';
  import { toast } from 'svelte-sonner';

  /**
   * @typedef {Object} Monitor
   * @property {number} id - The ID.
   * @property {boolean} active - The monitor active status.
   * @property {string} name - The monitor name.
   * @property {string} url - The monitor URL.
   * @property {string} monitor_type - The monitor type.
   * @property {number} retry - The number of retries.
   * @property {number} retry_interval - The retry interval in seconds.
   */

  /** @type {Monitor} */
  export let monitor;

  /** @type {boolean} */
  let deleteDialogOpen = false;

  function handleDelete() {
    deleteDialogOpen = false;
    const promise = new Promise((resolve, reject) =>
      // TODO: delete this monitor
      setTimeout(() => {
        if (Math.random() > 0.5) {
          resolve({ name: 'Svelte Sonner' });
        } else {
          reject('Not found');
        }
      }, 2500)
    );

    toast.promise(promise, {
      loading: 'Loading...',
      success: `Monitor "${monitor.name}" deleted`,
      error: (e) => {
        return `Error: ${e}`
      }
    });
  }
</script>

<Dialog.Root bind:open={deleteDialogOpen}>
  <DropdownMenu.Root>
    <DropdownMenu.Trigger asChild let:builder>
      <Button
        variant="ghost"
        builders={[builder]}
        class="flex h-8 w-8 p-0 data-[state=open]:bg-muted"
      >
        <EllipsisVertical class="h-4 w-4" />
        <span class="sr-only">Open Monitor Options</span>
      </Button>
    </DropdownMenu.Trigger>
    <DropdownMenu.Content class="w-auto" align="end">
      <DropdownMenu.Item href={`/monitors/${monitor.id}`}>Open</DropdownMenu.Item>
      <DropdownMenu.Item
        on:click={() =>
          toast.success('Monitor paused', {
            description: `Monitor "${monitor.name}" paused`,
            action: {
              label: 'Undo',
              onClick: () => console.info('Undo')
            }
          })}>Pause</DropdownMenu.Item
      >
      <DropdownMenu.Separator />
      <Dialog.Trigger class="w-full">
        <DropdownMenu.Item>Delete</DropdownMenu.Item>
      </Dialog.Trigger>
    </DropdownMenu.Content>
  </DropdownMenu.Root>
  <Dialog.Content class="sm:max-w-[425px]">
    <Dialog.Header>
      <Dialog.Title>Delete Monitor</Dialog.Title>
      <Dialog.Description>
        Are you sure you want to delete "{monitor.name}"?
      </Dialog.Description>
    </Dialog.Header>
    <Dialog.Footer>
      <Button variant="destructive" on:click={handleDelete}>Yes, Delete</Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>
