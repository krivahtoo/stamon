<script>
  import * as Tooltip from '$lib/components/ui/tooltip/index.js';
  import StatusCell from '$lib/components/table/status-cell.svelte';

  

  
  /**
   * @typedef {Object} Props
   * @property {string} message
   * @property {number} statusId
   */

  /** @type {Props} */
  let { message, statusId } = $props();
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
