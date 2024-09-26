<script>
  import * as Tooltip from '$lib/components/ui/tooltip/index.js';
  import { format } from 'date-fns';

  /** @type {string} incident date */
  export let date;
</script>

<Tooltip.Root>
  <Tooltip.Trigger class="flex space-x-2">
    {format(date, 'EEE do')}
  </Tooltip.Trigger>
  <Tooltip.Content>
    <p>{date}</p>
  </Tooltip.Content>
</Tooltip.Root>
