<script>
  import * as Tooltip from '$lib/components/ui/tooltip/index.js';
  import StatusCell from '$lib/components/table/status-cell.svelte';

  /** @type {string} message string */
  export let message;

  /** @type {number} status integer represention */
  export let statusId;
</script>

<Tooltip.Root>
  <Tooltip.Trigger class="flex space-x-2">
    <StatusCell {statusId} />
    <span class="max-w-[100px] truncate">
      {message}
    </span>
  </Tooltip.Trigger>
  <Tooltip.Content>
    <p>{message}</p>
  </Tooltip.Content>
</Tooltip.Root>
