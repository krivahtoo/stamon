<script>
  import EyeOff from '@lucide/svelte/icons/eye-off';
  import ArrowDownAZ from '@lucide/svelte/icons/arrow-down-a-z';
  import ArrowUpAZ from '@lucide/svelte/icons/arrow-up-a-z';
  import ChevronsUpDown from '@lucide/svelte/icons/chevrons-up-down';
  import Eraser from '@lucide/svelte/icons/eraser';
  import { cn } from '$lib/utils.js';
  import { Button } from '$lib/components/ui/button/index.js';
  import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';

  /**
   * @typedef {import("svelte-headless-table").TableViewModel<import('$lib/store/services').Service>} TableViewModel
   */

  /**
   * @typedef {Object} SortProps
   * @property {"desc" | "asc" | undefined} order
   * @property {(event: Event) => void} toggle
   * @property {() => void} clear
   * @property {boolean} disabled
   */

  /**
   * @typedef {Object} Props
   * @property {never} select
   * @property {SortProps} sort
   * @property {never} filter
   */

  /**
   * @type {{
   *   class?: string | null,
   *   props: Props,
   *   tableModel: TableViewModel,
   *   cellId: string,
   *   children?: import('svelte').Snippet
   * }}
   */
  let { class: className = undefined, props: colProps, tableModel, cellId, children } = $props();

  const { hiddenColumnIds } = tableModel.pluginStates.hide;

  /**
   * Handle ascending sort.
   * @param {Event} e
   */
  function handleAscSort(e) {
    if (colProps.sort.order === 'asc') {
      return;
    }
    colProps.sort.toggle(e);
  }

  /**
   * Handle descending sort.
   * @param {Event} e
   */
  function handleDescSort(e) {
    if (colProps.sort.order === 'desc') {
      return;
    }
    if (colProps.sort.order === undefined) {
      // We can only toggle, so we toggle from undefined to 'asc' first
      colProps.sort.toggle(e);
    }
    colProps.sort.toggle(e); // Then we toggle from 'asc' to 'desc'
  }

  function handleHide() {
    hiddenColumnIds.update((/** @type {string[]} */ ids) => {
      if (ids.includes(cellId)) {
        return ids;
      }
      return [...ids, cellId];
    });
  }
</script>

{#if !colProps.sort.disabled}
  <div class={cn('flex items-center', className)}>
    <DropdownMenu.Root>
      <DropdownMenu.Trigger>
        {#snippet child({ props })}
        <Button
          variant="ghost"
          {...props}
          class="-ml-3 h-8 data-[state=open]:bg-accent"
          size="sm"
        >
          {@render children?.()}
          {#if colProps.sort.order === 'desc'}
            <ArrowUpAZ class="ml-2 h-4 w-4" />
          {:else if colProps.sort.order === 'asc'}
            <ArrowDownAZ class="ml-2 h-4 w-4" />
          {:else}
            <ChevronsUpDown class="ml-2 h-4 w-4" />
          {/if}
        </Button>
        {/snippet}
      </DropdownMenu.Trigger>
      <DropdownMenu.Content align="start">
        <DropdownMenu.Item onSelect={handleAscSort}>
          <ArrowDownAZ class="mr-2 h-3.5 w-3.5 text-muted-foreground/70" />
          Asc
        </DropdownMenu.Item>
        <DropdownMenu.Item onSelect={handleDescSort}>
          <ArrowUpAZ class="mr-2 h-3.5 w-3.5 text-muted-foreground/70" />
          Desc
        </DropdownMenu.Item>
        <DropdownMenu.Separator />
        <DropdownMenu.Item onSelect={() => colProps.sort.clear()}>
          <Eraser class="mr-2 h-3.5 w-3.5 text-muted-foreground/70" />
          Clear
        </DropdownMenu.Item>
        <DropdownMenu.Separator />
        <DropdownMenu.Item onSelect={handleHide}>
          <EyeOff class="mr-2 h-3.5 w-3.5 text-muted-foreground/70" />
          Hide
        </DropdownMenu.Item>
      </DropdownMenu.Content>
    </DropdownMenu.Root>
  </div>
{:else}
  {@render children?.()}
{/if}
