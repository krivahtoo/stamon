<script>
  import { Button as ButtonPrimitive } from 'bits-ui';
  import { buttonVariants } from './index.js';
  import { cn } from '$lib/utils.js';
  let className = undefined;
  /** @type {import('tailwind-variants').VariantProps<typeof import('./index.js').buttonVariants>['variant']} */
  export let variant = 'default';
  /** @type {import('tailwind-variants').VariantProps<typeof import('./index.js').buttonVariants>['size']} */
  export let size = 'default';
  /** @type {import('bits-ui').Button.Props['builders']} */
  export let builders = [];
  export { className as class };
</script>

<ButtonPrimitive.Root
  {builders}
  class={cn(buttonVariants({ variant, size, className }))}
  type="button"
  {...$$restProps}
  on:click
  on:keydown
>
  <slot />
</ButtonPrimitive.Root>
