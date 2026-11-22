<script>
  import * as Tooltip from '$lib/components/ui/tooltip/index.js';

  /**
   * @typedef {Object} Props
   * @property {string} message
   */

  /** @type {Props} */
  let { message } = $props();
</script>

<Tooltip.Root>
  <Tooltip.Trigger class="block w-full truncate text-left">
    {message}
  </Tooltip.Trigger>
  <Tooltip.Content class="max-w-sm">
    <p>{message}</p>
  </Tooltip.Content>
</Tooltip.Root>
