<script>
  import { Label } from '$lib/components/ui/label/index.js';
  import { cn } from '$lib/utils.js';

  /**
   * A label, an optional tag, the control and a muted hint underneath.
   * @typedef {Object} Props
   * @property {string} label
   * @property {string} [id] - The control's id, for the label.
   * @property {boolean} [optional]
   * @property {string} [hint]
   * @property {string} [class]
   * @property {import('svelte').Snippet} children
   */

  /** @type {Props} */
  let { label, id, optional = false, hint, class: className, children } = $props();
</script>

<div class={cn('flex min-w-0 flex-col gap-1.5', className)}>
  <div class="flex items-baseline gap-2">
    <Label for={id} class="font-semibold">{label}</Label>
    {#if optional}
      <span class="text-xs text-muted-foreground">Optional</span>
    {/if}
  </div>
  {@render children()}
  {#if hint}
    <p class="text-xs text-muted-foreground">{hint}</p>
  {/if}
</div>
