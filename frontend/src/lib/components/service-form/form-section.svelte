<script>
  import * as Card from '$lib/components/ui/card/index.js';

  /**
   * A numbered form section: a card with the number, title and description on top.
   * @typedef {Object} Props
   * @property {number} number
   * @property {string} title
   * @property {string} description
   * @property {import('svelte').Snippet} children
   */

  /** @type {Props} */
  let { number, title, description, children } = $props();
</script>

<Card.Root class="gap-0 py-0">
  <header class="flex gap-3 border-b p-4">
    <span
      class="grid size-6 shrink-0 place-items-center rounded-full bg-raised text-xs font-semibold text-muted-foreground"
    >
      {number}
    </span>
    <div class="flex min-w-0 flex-col gap-0.5">
      <h2 class="font-semibold">{title}</h2>
      <p class="text-sm text-muted-foreground">{description}</p>
    </div>
  </header>
  <div class="p-4">
    {@render children()}
  </div>
</Card.Root>
