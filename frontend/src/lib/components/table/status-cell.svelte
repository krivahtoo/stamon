<script>
  import { Badge } from '$lib/components/ui/badge/index.js';
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
  <Badge variant={status.variant}>{status.label}</Badge>
{/if}
