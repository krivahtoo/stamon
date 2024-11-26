<script>
  import { cn } from '$lib/utils.js';

  /** @type {string} */
  let className = '';
  export { className as class };
</script>

<div class={cn('flex flex-col space-y-1.5 p-6', className)} {...$$restProps}>
  <slot />
</div>
