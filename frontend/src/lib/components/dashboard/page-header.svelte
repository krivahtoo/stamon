<script>
  import { cn } from '$lib/utils.js';

  /**
   * @typedef {Object} Props
   * @property {string} [label] - Small accent label above the title.
   * @property {string} title
   * @property {number | string} [count] - Shown in a badge next to the title.
   * @property {string} [description] - One muted line under the title.
   * @property {import('svelte').Snippet} [actions] - Page actions, on the right.
   * @property {string} [class]
   */

  /** @type {Props} */
  let { label, title, count, description, actions, class: className } = $props();
</script>

<header
  data-slot="page-header"
  class={cn('flex flex-col gap-4 sm:flex-row sm:items-end sm:justify-between', className)}
>
  <div class="flex min-w-0 flex-col gap-1">
    {#if label}
      <p class="text-xs font-bold tracking-[0.08em] text-primary uppercase">{label}</p>
    {/if}
    <div class="flex items-center gap-2.5">
      <h1 class="truncate text-2xl font-bold tracking-[-0.015em]">{title}</h1>
      {#if count !== undefined}
        <span
          class="rounded-full bg-raised px-2 py-px text-xs font-semibold text-muted-foreground tabular-nums"
        >
          {count}
        </span>
      {/if}
    </div>
    {#if description}
      <p class="text-sm/[1.65] text-muted-foreground sm:text-base/[1.65]">{description}</p>
    {/if}
  </div>
  {#if actions}
    <!-- Actions fill the width on mobile and hug their content from sm up. -->
    <div class="flex flex-wrap gap-2 *:flex-1 sm:shrink-0 sm:*:flex-none">
      {@render actions()}
    </div>
  {/if}
</header>
