<script>
  import { StatusPill } from '$lib/components/dashboard/index.js';
  import { statuses, StatusValue } from '$lib/data/table.js';

  
  /**
   * @typedef {Object} Props
   * @property {number | undefined} [statusId]
   */

  /** @type {Props} */
  let { statusId = undefined } = $props();

  let status = $derived(statuses.find((status) => status.value === statusId) ?? statuses[StatusValue.Pending]);
</script>

{#if status}
  <StatusPill tone={status.tone}>{status.label}</StatusPill>
{/if}
