<script>
  import { cn } from '$lib/utils.js';
  let { ref = $bindable(null), class: className, children, ...restProps } = $props();
</script>

<div
  bind:this={ref}
  data-slot="card-footer"
  class={cn(
    'flex items-center rounded-b-lg px-(--card-spacing) [.border-t]:pt-(--card-spacing)',
    className
  )}
  {...restProps}
>
  {@render children?.()}
</div>
