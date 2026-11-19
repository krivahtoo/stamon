<script>
  import { cn } from '$lib/utils.js';
  let { ref = $bindable(null), class: className, children, ...restProps } = $props();
</script>

<div
  bind:this={ref}
  role="list"
  data-slot="item-group"
  class={cn(
    'group/item-group flex w-full flex-col gap-4 has-data-[size=sm]:gap-2.5 has-data-[size=xs]:gap-2',
    className
  )}
  {...restProps}
>
  {@render children?.()}
</div>
