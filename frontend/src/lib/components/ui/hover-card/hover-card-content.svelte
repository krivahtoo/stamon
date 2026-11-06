<script>
  import { LinkPreview as HoverCardPrimitive } from 'bits-ui';
  import { cn, flyAndScale } from '$lib/utils.js';
  
  /**
   * @typedef {Object} Props
   * @property {any} [class]
   * @property {any} [transition]
   * @property {any} [transitionConfig]
   * @property {'start' | 'center' | 'end'} [align]
   * @property {number} [sideOffset]
   * @property {import('svelte').Snippet} [children]
   */

  /** @type {Props & { [key: string]: any }} */
  let {
    class: className = undefined,
    transition = flyAndScale,
    transitionConfig = undefined,
    align = 'center',
    sideOffset = 4,
    children,
    ...rest
  } = $props();
  
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
  {...rest}
>
  {@render children?.()}
</HoverCardPrimitive.Content>
