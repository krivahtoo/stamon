<script>
  import { onMount } from 'svelte';
  import { cfetch } from '$lib/utils.js';
  import { toDate, formatRFC7231 } from 'date-fns';

  import { UptimeBar } from '$lib/components/dashboard/index.js';
  import { statuses as statusList, StatusValue } from '$lib/data/table.js';

  
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

  let ticks = $derived(
    statuses.map((log) => ({
      tone: statusList.find((status) => status.value === log.status)?.tone ?? 'empty',
      label: formatRFC7231(toDate(log.time))
    }))
  );
  let uptime = $derived(
    statuses.length
      ? (statuses.filter((log) => log.status === StatusValue.Up).length / statuses.length) * 100
      : undefined
  );

  onMount(async () => {
    const res_log = await cfetch(`/services/${serviceId}/logs?limit=32`);

    const logs = await res_log.json();
    // console.log(logs.logs);
    statuses = logs.logs.reverse();
  });
</script>

<UptimeBar {ticks} {uptime} />
