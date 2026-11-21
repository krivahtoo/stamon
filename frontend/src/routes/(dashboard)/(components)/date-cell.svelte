<script>
  import * as Tooltip from '$lib/components/ui/tooltip/index.js';
  import { format } from 'date-fns';
  import { formatStarted } from './format.js';

  /**
   * @typedef {Object} Props
   * @property {string} start
   */

  /** @type {Props} */
  let { start } = $props();
</script>

<Tooltip.Root>
  <Tooltip.Trigger class="tabular-nums">
    {formatStarted(start)}
  </Tooltip.Trigger>
  <Tooltip.Content>
    <p>{format(start, 'PPpp')}</p>
  </Tooltip.Content>
</Tooltip.Root>
