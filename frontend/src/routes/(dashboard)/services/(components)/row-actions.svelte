<script>
  import Ellipsis from '@lucide/svelte/icons/ellipsis';

  import { Button } from '$lib/components/ui/button/index.js';
  import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
  import * as Dialog from '$lib/components/ui/dialog/index.js';
  import { deleteService, togglePause } from '$lib/data/service-actions.js';

  /**
   * @typedef {Object} Props
   * @property {import('$lib/store/services').Service} service
   */

  /** @type {Props} */
  let { service } = $props();

  /** @type {boolean} */
  let deleteDialogOpen = $state(false);

  function handleDelete() {
    deleteDialogOpen = false;
    // The toast reports failures.
    deleteService(service).catch(() => {});
  }

  function handlePause() {
    togglePause(service).catch(() => {});
  }
</script>

<Dialog.Root bind:open={deleteDialogOpen}>
  <DropdownMenu.Root>
    <DropdownMenu.Trigger>
      {#snippet child({ props })}
        <Button
          variant="ghost"
          size="icon-sm"
          {...props}
          class="text-muted-foreground data-[state=open]:bg-raised"
        >
          <Ellipsis />
          <span class="sr-only">Actions for {service.name}</span>
        </Button>
      {/snippet}
    </DropdownMenu.Trigger>
    <DropdownMenu.Content class="w-auto" align="end">
      <DropdownMenu.Item>
        {#snippet child({ props })}
          <a href="/services/{service.id}" {...props}>Open</a>
        {/snippet}
      </DropdownMenu.Item>
      <DropdownMenu.Item onSelect={handlePause}>
        {service.active ? 'Pause' : 'Resume'}
      </DropdownMenu.Item>
      <DropdownMenu.Separator />
      <DropdownMenu.Item variant="destructive" onSelect={() => (deleteDialogOpen = true)}>
        Delete
      </DropdownMenu.Item>
    </DropdownMenu.Content>
  </DropdownMenu.Root>
  <Dialog.Content class="sm:max-w-[425px]">
    <Dialog.Header>
      <Dialog.Title>Delete service</Dialog.Title>
      <Dialog.Description>
        Are you sure you want to delete "{service.name}"?
      </Dialog.Description>
    </Dialog.Header>
    <Dialog.Footer>
      <Button variant="destructive" onclick={handleDelete}>Yes, delete</Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>
