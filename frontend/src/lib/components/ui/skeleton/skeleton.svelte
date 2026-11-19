<script>
  import { cn } from '$lib/utils.js';
  let { ref = $bindable(null), class: className, ...restProps } = $props();
</script>

<div
  bind:this={ref}
  data-slot="skeleton"
  class={cn('animate-pulse rounded-md bg-muted', className)}
  {...restProps}
></div>
