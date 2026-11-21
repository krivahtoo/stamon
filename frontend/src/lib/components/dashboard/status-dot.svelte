<script>
  import { cn } from '$lib/utils.js';
  import { toneFill } from './tones.js';

  /**
   * @typedef {Object} Props
   * @property {import('./tones.js').Tone} [tone]
   * @property {'sm' | 'default'} [size] - 6px inside pills, 8px elsewhere.
   * @property {string} [class]
   */

  /** @type {Props} */
  let { tone = 'neutral', size = 'default', class: className } = $props();
</script>

<span
  aria-hidden="true"
  class={cn(
    'inline-block shrink-0 rounded-full',
    size === 'sm' ? 'size-1.5' : 'size-2',
    toneFill[tone],
    className
  )}
></span>
