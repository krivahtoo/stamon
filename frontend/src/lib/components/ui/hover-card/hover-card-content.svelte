<script>
  import { LinkPreview as HoverCardPrimitive } from 'bits-ui';
  import { cn } from '$lib/utils.js';

  /** @type {any} */
  let {
    ref = $bindable(null),
    class: className,
    align = 'center',
    sideOffset = 4,
    portalProps,
    ...restProps
  } = $props();
</script>

<HoverCardPrimitive.Portal {...portalProps}>
  <HoverCardPrimitive.Content
    bind:ref
    {align}
    {sideOffset}
    class={cn(
      'z-50 mt-3 w-64 animate-in rounded-md border bg-popover p-4 text-popover-foreground shadow-md fade-in-0 outline-none zoom-in-95 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95',
      className
    )}
    {...restProps}
  />
</HoverCardPrimitive.Portal>
