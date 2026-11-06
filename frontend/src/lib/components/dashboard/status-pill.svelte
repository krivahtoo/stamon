<script>
  import { cn } from '$lib/utils.js';
  import StatusDot from './status-dot.svelte';
  import { toneTint } from './tones.js';

  /**
   * @typedef {Object} Props
   * @property {import('./tones.js').Tone} [tone]
   * @property {string} [class]
   * @property {import('svelte').Snippet} [children] - The label.
   */

  /** @type {Props} */
  let { tone = 'neutral', class: className, children } = $props();
</script>

<span
  data-slot="status-pill"
  class={cn(
    'inline-flex w-fit shrink-0 items-center gap-1.5 rounded-full px-2 py-0.5 text-xs font-semibold whitespace-nowrap text-foreground',
    toneTint[tone],
    className
  )}
>
  <StatusDot {tone} size="sm" />
  {@render children?.()}
</span>
