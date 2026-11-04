<script>
  import { Dialog as DialogPrimitive } from 'bits-ui';
  /**
   * @typedef {Object} Props
   * @property {import('svelte').Snippet} [children]
   */

  /** @type {Props & { [key: string]: any }} */
  let { children, ...rest } = $props();
</script>

<DialogPrimitive.Portal {...rest}>
  {@render children?.()}
</DialogPrimitive.Portal>
