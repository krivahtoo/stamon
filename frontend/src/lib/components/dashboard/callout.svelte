<script>
  import CircleAlert from '@lucide/svelte/icons/circle-alert';
  import CircleCheck from '@lucide/svelte/icons/circle-check';
  import Info from '@lucide/svelte/icons/info';
  import { cn } from '$lib/utils.js';
  import { toneBorder, toneText, toneTint } from './tones.js';

  /** @type {Record<import('./tones.js').Tone, import('svelte').Component<any>>} */
  const defaultIcons = {
    up: CircleCheck,
    down: CircleAlert,
    pending: CircleAlert,
    maintenance: Info,
    neutral: Info,
    empty: Info
  };

  /**
   * @typedef {Object} Props
   * @property {import('./tones.js').Tone} [tone]
   * @property {string} [title]
   * @property {import('svelte').Component<any>} [icon] - Defaults to one that fits the tone.
   * @property {string} [class]
   * @property {import('svelte').Snippet} [children] - The body.
   */

  /** @type {Props} */
  let { tone = 'maintenance', title, icon, class: className, children } = $props();

  let Icon = $derived(icon ?? defaultIcons[tone]);
</script>

<div
  data-slot="callout"
  role={tone === 'down' ? 'alert' : 'status'}
  class={cn(
    'flex gap-3 rounded-md border-l-[3px] px-4 py-3 text-sm',
    toneTint[tone],
    toneBorder[tone],
    className
  )}
>
  <Icon class={cn('mt-px size-[18px] shrink-0', toneText[tone])} aria-hidden="true" />
  <div class="flex min-w-0 flex-col gap-0.5">
    {#if title}
      <p class="font-semibold text-foreground">{title}</p>
    {/if}
    {#if children}
      <div class="leading-[1.65] text-muted-foreground">{@render children()}</div>
    {/if}
  </div>
</div>
