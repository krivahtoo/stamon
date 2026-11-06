<script>
  import { cn } from '$lib/utils.js';

  /**
   * @typedef {Object} Props
   * @property {import('svelte').Component<any>} icon - A Lucide outline icon.
   * @property {string} message - One sentence.
   * @property {boolean} [framed] - Draw the card border. Turn off inside an existing card.
   * @property {string} [class]
   * @property {import('svelte').Snippet} [children] - The primary action.
   */

  /** @type {Props} */
  let { icon: Icon, message, framed = true, class: className, children } = $props();
</script>

<div
  data-slot="empty-state"
  class={cn(
    'flex flex-col items-center gap-4 px-6 py-12 text-center',
    framed && 'rounded-xl border bg-card shadow-card',
    className
  )}
>
  <div class="flex size-12 items-center justify-center rounded-full bg-raised">
    <Icon class="size-6 text-muted-foreground" aria-hidden="true" />
  </div>
  <p class="max-w-md text-sm text-balance text-muted-foreground">{message}</p>
  {@render children?.()}
</div>
