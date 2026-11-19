<script>
  import * as Tooltip from '$lib/components/ui/tooltip/index.js';
  import { format, formatRelative } from 'date-fns';
  import { formatDuration } from './format.js';

  /**
   * @typedef {Object} Props
   * @property {string} start
   * @property {string} end
   */

  /** @type {Props} */
  let { start, end } = $props();
</script>

<Tooltip.Root>
  <Tooltip.Trigger>
    {formatDuration(start, end)}
  </Tooltip.Trigger>
  <Tooltip.Content>
    <p>{formatRelative(start, new Date())} to {format(end, 'p')}</p>
  </Tooltip.Content>
</Tooltip.Root>
