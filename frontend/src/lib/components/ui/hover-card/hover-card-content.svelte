<script>
  import { LinkPreview as HoverCardPrimitive } from 'bits-ui';
  import { cn, flyAndScale } from '$lib/utils.js';
  let className = undefined;
  export let transition = flyAndScale;
  export let transitionConfig = undefined;
  export let align = 'center';
  export let sideOffset = 4;
  export { className as class };
</script>

<HoverCardPrimitive.Content
  {transition}
  {transitionConfig}
  {align}
  {sideOffset}
  class={cn(
    'z-50 mt-3 w-64 rounded-md border bg-popover p-4 text-popover-foreground shadow-md outline-none',
    className
  )}
  {...$$restProps}
>
  <slot />
</HoverCardPrimitive.Content>
