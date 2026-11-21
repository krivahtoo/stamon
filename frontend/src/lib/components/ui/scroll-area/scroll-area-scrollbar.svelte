<script>
  import { ScrollArea as ScrollAreaPrimitive } from 'bits-ui';
  import { cn } from '$lib/utils.js';

  /** @type {any} */
  let {
    ref = $bindable(null),
    class: className,
    orientation = 'vertical',
    children,
    ...restProps
  } = $props();
</script>

<ScrollAreaPrimitive.Scrollbar
  bind:ref
  {orientation}
  class={cn(
    'flex touch-none transition-colors select-none',
    orientation === 'vertical' && 'h-full w-2.5 border-l border-l-transparent p-px',
    orientation === 'horizontal' && 'h-2.5 w-full border-t border-t-transparent p-px',
    className
  )}
  {...restProps}
>
  {@render children?.()}
  <ScrollAreaPrimitive.Thumb
    class={cn('relative rounded-full bg-border', orientation === 'vertical' && 'flex-1')}
  />
</ScrollAreaPrimitive.Scrollbar>
