<script>
  import { DropdownMenu as DropdownMenuPrimitive } from 'bits-ui';
  /** @type {string | undefined} */
  export let value = undefined;
</script>

<DropdownMenuPrimitive.RadioGroup {...$$restProps} bind:value>
  <slot />
</DropdownMenuPrimitive.RadioGroup>
