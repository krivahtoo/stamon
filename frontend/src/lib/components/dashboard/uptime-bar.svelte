<script>
  import { cn } from '$lib/utils.js';
  import * as Tooltip from '$lib/components/ui/tooltip/index.js';
  import { toneFill } from './tones.js';

  /**
   * @typedef {Object} Tick
   * @property {import('./tones.js').Tone} tone
   * @property {string} [label] - Shown in a tooltip, e.g. the check time.
   */

  /**
   * @typedef {Object} Props
   * @property {Tick[]} ticks - Oldest first.
   * @property {number} [uptime] - Percentage shown next to the ticks.
   * @property {string} [class]
   */

  /** @type {Props} */
  let { ticks, uptime, class: className } = $props();
</script>

<div data-slot="uptime-bar" class={cn('flex min-w-0 items-center gap-3', className)}>
  <!-- Ticks wrap onto a second row when there isn't room, e.g. on mobile. -->
  <div class="flex min-w-30 flex-wrap gap-0.5" role="img" aria-label="Recent checks">
    {#each ticks as tick, i (i)}
      {#if tick.label}
        <Tooltip.Root>
          <Tooltip.Trigger
            aria-label={tick.label}
            class={cn('block h-5 w-[3px] rounded-[1px]', toneFill[tick.tone])}
          />
          <Tooltip.Content>
            <p>{tick.label}</p>
          </Tooltip.Content>
        </Tooltip.Root>
      {:else}
        <span class={cn('block h-5 w-[3px] rounded-[1px]', toneFill[tick.tone])}></span>
      {/if}
    {/each}
  </div>
  {#if uptime !== undefined}
    <span class="shrink-0 font-mono text-sm font-bold tabular-nums">{uptime.toFixed(2)}%</span>
  {/if}
</div>
