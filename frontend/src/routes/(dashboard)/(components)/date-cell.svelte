<script>
  import * as Tooltip from '$lib/components/ui/tooltip/index.js';
  import { format } from 'date-fns';

  
  /**
   * @typedef {Object} Props
   * @property {string} date
   */

  /** @type {Props} */
  let { date } = $props();
</script>

<Tooltip.Root>
  <Tooltip.Trigger class="flex space-x-2">
    {format(date, 'EEE do')}
  </Tooltip.Trigger>
  <Tooltip.Content>
    <p>{date}</p>
  </Tooltip.Content>
</Tooltip.Root>
