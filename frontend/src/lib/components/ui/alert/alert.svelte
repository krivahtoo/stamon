<script>
  import { alertVariants } from './index.js';
  import { cn } from '$lib/utils.js';
  let className = undefined;
  export let variant = 'default';
  export { className as class };
</script>

<div class={cn(alertVariants({ variant }), className)} {...$$restProps} role="alert">
  <slot />
</div>
