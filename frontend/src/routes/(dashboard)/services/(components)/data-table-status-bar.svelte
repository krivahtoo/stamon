<script>
  import { onMount } from 'svelte';
  import { cfetch } from '$lib/utils.js';
  import { toDate, formatRFC7231 } from 'date-fns';

  import * as Tooltip from '$lib/components/ui/tooltip/index.js';

  
  /**
   * @typedef {Object} Props
   * @property {number} serviceId
   */

  /** @type {Props} */
  let { serviceId } = $props();

  /**
   * @typedef {Object} Log
   * @property {number} service_id - The id of the service.
   * @property {number} status - The status.
   * @property {string} time - Timestamp.
   * @property {number} duration - Time taken.
   */

  /** @type {Log[]} */
  let statuses = $state([]);
  // Define the tailwind classes for each status
  /** @type {string[]} */
  const statusClasses = [
    'bg-raised',
    'bg-status-up',
    'bg-status-down',
    'bg-status-pending',
    'bg-status-maintenance'
  ];

  onMount(async () => {
    const res_log = await cfetch(`/services/${serviceId}/logs?limit=32`);

    const logs = await res_log.json();
    // console.log(logs.logs);
    statuses = logs.logs.reverse();
  });
</script>

<div class="flex space-x-1">
  {#each statuses as status, i}
    <Tooltip.Root>
      <Tooltip.Trigger>
        <div
          class={`hidden ${i > 20 ? 'sm:flex' : i > 10 ? 'md:flex' : 'lg:flex'} h-5 w-2 rounded-md ${statusClasses[status.status]}`}
        ></div>
      </Tooltip.Trigger>
      <Tooltip.Content>
        <p>{formatRFC7231(toDate(status.time))}</p>
      </Tooltip.Content>
    </Tooltip.Root>
  {/each}
</div>
