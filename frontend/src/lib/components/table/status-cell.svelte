<script>
  import { Badge } from '$lib/components/ui/badge/index.js';
  import { statuses, StatusValue } from '$lib/data/table.js';

  /** @type {number | undefined} The ID of the status, used to determine the status icon */
  export let statusId = undefined;

  $: status = statuses.find((status) => status.value === statusId) ?? statuses[StatusValue.Pending];
</script>

{#if status}
  <Badge variant={status.variant}>{status.label}</Badge>
{/if}
