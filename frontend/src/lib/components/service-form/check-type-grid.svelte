<script>
  import Box from '@lucide/svelte/icons/box';
  import CircleCheck from '@lucide/svelte/icons/circle-check';
  import Database from '@lucide/svelte/icons/database';
  import Globe from '@lucide/svelte/icons/globe';
  import HeartPulse from '@lucide/svelte/icons/heart-pulse';
  import Network from '@lucide/svelte/icons/network';
  import Plug from '@lucide/svelte/icons/plug';
  import Radar from '@lucide/svelte/icons/radar';
  import ShieldCheck from '@lucide/svelte/icons/shield-check';
  import { cn } from '$lib/utils.js';
  import { checkKinds, kindOf, setKind } from './service-form.js';

  /**
   * @typedef {Object} Props
   * @property {import('./service-form.js').ServiceModel} model
   */

  /** @type {Props} */
  let { model = $bindable() } = $props();

  /** @type {Record<string, typeof Globe>} */
  const icons = {
    http: Globe,
    ping: Radar,
    tcp: Plug,
    dns: Network,
    tls: ShieldCheck,
    docker: Box,
    postgres: Database,
    mysql: Database,
    redis: Database,
    push: HeartPulse
  };

  let selected = $derived(kindOf(model));

  /**
   * Arrow keys move between the cards, as in a radio group.
   * @param {KeyboardEvent} event
   * @param {number} index
   */
  function onkeydown(event, index) {
    const step = { ArrowRight: 1, ArrowDown: 1, ArrowLeft: -1, ArrowUp: -1 }[event.key];
    if (!step) return;
    event.preventDefault();
    const next = (index + step + checkKinds.length) % checkKinds.length;
    setKind(model, checkKinds[next].value);
    const group = /** @type {HTMLElement} */ (event.currentTarget).parentElement;
    /** @type {HTMLElement | undefined} */ (group?.children[next])?.focus();
  }
</script>

<div role="radiogroup" aria-label="Check type" class="grid grid-cols-2 gap-3 lg:grid-cols-3">
  {#each checkKinds as kind, index (kind.value)}
    {@const Icon = icons[kind.value]}
    {@const checked = selected === kind.value}
    <button
      type="button"
      role="radio"
      aria-checked={checked}
      tabindex={checked ? 0 : -1}
      class={cn(
        'relative flex flex-col items-start gap-1 rounded-lg border bg-background p-3 text-left transition-colors duration-160 hover:bg-surface',
        checked && 'border-primary bg-primary/5 hover:bg-primary/5'
      )}
      onclick={() => setKind(model, kind.value)}
      onkeydown={(e) => onkeydown(e, index)}
    >
      <span
        class={cn(
          'mb-1 grid size-7 place-items-center rounded-md bg-raised text-muted-foreground',
          checked && 'bg-primary text-primary-foreground'
        )}
      >
        <Icon class="size-4" />
      </span>
      <span class="text-sm font-semibold">{kind.label}</span>
      <span class="text-xs text-muted-foreground">{kind.description}</span>
      {#if checked}
        <CircleCheck class="absolute top-3 right-3 size-4 text-primary" />
      {/if}
    </button>
  {/each}
</div>
