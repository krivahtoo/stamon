<script>
  import CalendarDays from '@lucide/svelte/icons/calendar-days';
  import * as Popover from '$lib/components/ui/popover/index.js';
  import { Button } from '$lib/components/ui/button/index.js';
  import { Checkbox } from '$lib/components/ui/checkbox/index.js';
  import { formatDay } from './format.js';

  /**
   * @typedef {Object} Props
   * @property {string[]} days - `YYYY-MM-DD` days to choose from, newest first.
   * @property {string[]} value - The selected days. Empty shows every day.
   */

  /** @type {Props} */
  let { days, value = $bindable([]) } = $props();

  /** @param {string} day */
  function toggle(day) {
    value = value.includes(day) ? value.filter((d) => d !== day) : [...value, day];
  }
</script>

<Popover.Root>
  <Popover.Trigger>
    {#snippet child({ props })}
      <Button {...props} variant="outline">
        <CalendarDays />
        Date
        {#if value.length}
          <span
            class="rounded-full bg-raised px-1.5 text-xs font-semibold text-muted-foreground tabular-nums"
          >
            {value.length}
          </span>
        {/if}
      </Button>
    {/snippet}
  </Popover.Trigger>
  <Popover.Content align="end" class="w-56 p-1">
    {#each days as day (day)}
      <label
        class="flex cursor-pointer items-center gap-2.5 rounded-md px-2.5 py-2 text-sm hover:bg-surface"
      >
        <Checkbox checked={value.includes(day)} onCheckedChange={() => toggle(day)} />
        {formatDay(day)}
      </label>
    {:else}
      <p class="px-2.5 py-2 text-sm text-muted-foreground">No days to filter by.</p>
    {/each}
    {#if value.length}
      <div class="mt-1 border-t pt-1">
        <Button variant="ghost" size="sm" class="w-full" onclick={() => (value = [])}>
          Clear filter
        </Button>
      </div>
    {/if}
  </Popover.Content>
</Popover.Root>
