<script>
  import { alertVariants } from './index.js';
  import { cn } from '$lib/utils.js';
  let className = undefined;
  /** @type {import('tailwind-variants').VariantProps<typeof import('./index.js').alertVariants>['variant']} */
  export let variant = 'default';
  export { className as class };
</script>

<div class={cn(alertVariants({ variant }), className)} {...$$restProps} role="alert">
  <slot />
</div>
