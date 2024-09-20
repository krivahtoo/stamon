<script>
  import * as Tooltip from '$lib/components/ui/tooltip/index.js';
  import { formatDistance, format, formatRelative } from 'date-fns';

  /** @type {string} start time */
  export let start;
  /** @type {string} end time */
  export let end;
</script>

<Tooltip.Root>
  <Tooltip.Trigger class="flex space-x-2">
    {formatDistance(start, end)}
  </Tooltip.Trigger>
  <Tooltip.Content>
    <p>{formatRelative(start, new Date())} to {format(end, 'p')}</p>
  </Tooltip.Content>
</Tooltip.Root>
