<script>
  import { cn } from '$lib/utils.js';
  let { ref = $bindable(null), class: className, children, ...restProps } = $props();
</script>

<p
  bind:this={ref}
  data-slot="item-description"
  class={cn(
    'line-clamp-2 text-left text-xs/relaxed font-normal text-muted-foreground [&>a]:underline [&>a]:underline-offset-4 [&>a:hover]:text-primary',
    className
  )}
  {...restProps}
>
  {@render children?.()}
</p>
