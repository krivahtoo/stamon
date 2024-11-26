<script>
  import EllipsisVertical from 'lucide-svelte/icons/ellipsis-vertical';
  import { toast } from 'svelte-sonner';

  import { Button } from '$lib/components/ui/button/index.js';
  import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
  import * as Dialog from '$lib/components/ui/dialog/index.js';
  import { cfetch } from '$lib/utils.js';

  /** @type {import('$lib/store/services').Service} */
  export let service;

  /** @type {boolean} */
  let deleteDialogOpen = false;

  function handleDelete() {
    deleteDialogOpen = false;
    const promise = new Promise((resolve, reject) =>
      cfetch(`/services/${service.id}`, { method: 'DELETE' }).then(async (res) => {
        if (res.ok) {
          const data = await res.json();
          // console.log(data);
          resolve(data);
        } else {
          reject(res.statusText);
        }
      })
    );

    toast.promise(promise, {
      loading: 'Loading...',
      success: `Monitor "${service.name}" deleted`,
      error: (e) => {
        return `Error: ${e}`;
      }
    });
  }

  function handlePause() {
    const promise = new Promise((resolve, reject) =>
      cfetch(`/services/${service.id}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ active: !service.active })
      })
        .then(async (res) => {
          if (res.ok) {
            const data = await res.json();
            // console.log(data);
            resolve(data);
          } else {
            reject(res.statusText);
          }
        })
        .catch((e) => {
          reject(e);
        })
    );

    toast.promise(promise, {
      loading: 'Loading...',
      success: `Monitor "${service.name}" paused`,
      error: (e) => {
        return `Error: ${e}`;
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
      <DropdownMenu.Item href={`/services/${service.id}`}>Open</DropdownMenu.Item>
      <DropdownMenu.Item on:click={handlePause}>
        {#if service.active}
          Pause
        {:else}
          Resume
        {/if}
      </DropdownMenu.Item>
      <DropdownMenu.Separator />
      <Dialog.Trigger class="w-full">
        <DropdownMenu.Item>Delete</DropdownMenu.Item>
      </Dialog.Trigger>
    </DropdownMenu.Content>
  </DropdownMenu.Root>
  <Dialog.Content class="sm:max-w-[425px]">
    <Dialog.Header>
      <Dialog.Title>Delete Service</Dialog.Title>
      <Dialog.Description>
        Are you sure you want to delete "{service.name}"?
      </Dialog.Description>
    </Dialog.Header>
    <Dialog.Footer>
      <Button variant="destructive" on:click={handleDelete}>Yes, Delete</Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>
