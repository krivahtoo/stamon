<script>
  import { DropdownMenu as DropdownMenuPrimitive } from 'bits-ui';
  
  /**
   * @typedef {Object} Props
   * @property {string | undefined} [value]
   * @property {import('svelte').Snippet} [children]
   */

  /** @type {Props & { [key: string]: any }} */
  let { value = $bindable(undefined), children, ...rest } = $props();
</script>

<DropdownMenuPrimitive.RadioGroup {...rest} bind:value>
  {@render children?.()}
</DropdownMenuPrimitive.RadioGroup>
