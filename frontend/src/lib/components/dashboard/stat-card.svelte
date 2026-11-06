<script>
  import { cn } from '$lib/utils.js';
  import * as Card from '$lib/components/ui/card/index.js';
  import StatusDot from './status-dot.svelte';

  /**
   * @typedef {Object} Props
   * @property {string} label
   * @property {number | string} value
   * @property {string} [context] - One muted line under the value.
   * @property {import('./tones.js').Tone} [tone] - Colour of the status dot.
   * @property {string} [class]
   */

  /** @type {Props} */
  let { label, value, context, tone, class: className } = $props();
</script>

<Card.Root class={cn('min-w-0 gap-1.5 p-4 sm:gap-2 sm:p-5', className)}>
  <div class="flex items-center justify-between gap-2">
    <p class="truncate text-xs font-bold tracking-[0.08em] text-muted-foreground uppercase">
      {label}
    </p>
    {#if tone}
      <StatusDot {tone} />
    {/if}
  </div>
  <p class="text-2xl font-bold tracking-[-0.015em] tabular-nums sm:text-[2rem]">{value}</p>
  {#if context}
    <p class="truncate text-xs text-muted-foreground sm:text-sm">{context}</p>
  {/if}
</Card.Root>
