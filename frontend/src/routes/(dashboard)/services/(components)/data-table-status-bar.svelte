<script>
  import { onMount } from 'svelte';

  import { UptimeBar } from '$lib/components/dashboard/index.js';
  import { loadRecentChecks } from '$lib/data/checks.js';

  /**
   * @typedef {Object} Props
   * @property {number} serviceId
   */

  /** @type {Props} */
  let { serviceId } = $props();

  /** @type {import('$lib/data/checks.js').RecentChecks} */
  let checks = $state({ ticks: [] });

  onMount(async () => {
    checks = await loadRecentChecks(serviceId);
  });
</script>

<UptimeBar ticks={checks.ticks} uptime={checks.uptime} />
