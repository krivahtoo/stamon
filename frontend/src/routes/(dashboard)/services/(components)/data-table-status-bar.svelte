<script>
  import { onMount } from 'svelte';
  import { dev } from '$app/environment';
  import { cfetch } from '$lib/utils.js';
  import { toDate, formatRFC7231 } from 'date-fns';

  import * as Tooltip from '$lib/components/ui/tooltip/index.js';

  /** @type {number} */
  export let serviceId;

  /**
   * @typedef {Object} Log
   * @property {number} service_id - The id of the service.
   * @property {number} status - The status.
   * @property {string} time - Timestamp.
   * @property {number} duration - Time taken.
   */

  /** @type {Log[]} */
  let statuses = [];
  // Define the tailwind classes for each status
  /** @type {string[]} */
  const statusClasses = [
    'bg-gray-500/30 dark:bg-gray-900/80',
    'bg-green-500 dark:bg-green-500/70',
    'bg-red-500 dark:bg-red-500/70',
    'bg-yellow-500'
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