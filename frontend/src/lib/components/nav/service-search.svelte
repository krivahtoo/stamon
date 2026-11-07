<script>
  import { fuse } from '@sveu/extend/fuse';
  import Search from '@lucide/svelte/icons/search';
  import { goto } from '$app/navigation';
  import services from '$lib/store/services.js';
  import { cn } from '$lib/utils.js';

  /**
   * @typedef {Object} Props
   * @property {HTMLInputElement | null} [ref] - The search input
   * @property {boolean} [inline] - Show results in the flow (in a dialog) instead of a dropdown
   * @property {() => void} [onnavigate] - Called after going to a result
   * @property {string} [class]
   */

  /** @type {Props} */
  let { ref = $bindable(null), inline = false, onnavigate, class: className } = $props();

  const listId = $props.id();

  let search = $state('');
  let focused = $state(false);
  let highlighted = $state(0);

  /** @type {{ item: import('$lib/store/services.js').Service, refIndex: number }[]} */
  let results = $derived(
    fuse(search, $services, {
      fuseOptions: { threshold: 0.35, minMatchCharLength: 2, keys: ['name', 'target', 'tags'] },
      matchWhenEmpty: false,
      limit: 8
    }).results
  );
  let open = $derived((inline || focused) && search.trim().length > 0);

  // Show the platform's shortcut: ⌘K on Apple devices, Ctrl K elsewhere.
  let shortcut = $state('Ctrl K');
  $effect(() => {
    if (/Mac|iPhone|iPad/.test(navigator.platform)) shortcut = '⌘K';
  });

  $effect(() => {
    // Reset the highlight whenever the query changes.
    search;
    highlighted = 0;
  });

  /** @param {number} id */
  function select(id) {
    search = '';
    ref?.blur();
    goto(`/services/${id}/`);
    onnavigate?.();
  }

  /** @param {KeyboardEvent} e */
  function onkeydown(e) {
    if (e.key === 'Escape') {
      search = '';
      ref?.blur();
    } else if (!results.length) {
      return;
    } else if (e.key === 'ArrowDown') {
      e.preventDefault();
      highlighted = (highlighted + 1) % results.length;
    } else if (e.key === 'ArrowUp') {
      e.preventDefault();
      highlighted = (highlighted - 1 + results.length) % results.length;
    } else if (e.key === 'Enter') {
      e.preventDefault();
      select(results[highlighted].item.id);
    }
  }
</script>

<div class={cn('relative', className)}>
  <div
    class="flex h-9 items-center gap-2 rounded-md border bg-surface pr-1.5 pl-2.5 focus-within:outline-2 focus-within:outline-offset-3 focus-within:outline-ring"
  >
    <Search class="size-4 shrink-0 text-muted-foreground" />
    <input
      bind:this={ref}
      bind:value={search}
      type="search"
      placeholder="Search services…"
      autocomplete="off"
      role="combobox"
      aria-label="Search services"
      aria-expanded={open}
      aria-controls={listId}
      aria-activedescendant={open && results.length ? `${listId}-${highlighted}` : undefined}
      class="h-full min-w-0 flex-1 bg-transparent text-sm outline-none placeholder:text-muted-foreground [&::-webkit-search-cancel-button]:hidden"
      onfocus={() => (focused = true)}
      onblur={() => (focused = false)}
      {onkeydown}
    />
    {#if !inline}
      <kbd
        class="rounded bg-raised px-1.5 py-0.5 font-mono text-xs text-muted-foreground"
        aria-hidden="true"
      >
        {shortcut}
      </kbd>
    {/if}
  </div>

  {#if open}
    <ul
      id={listId}
      role="listbox"
      aria-label="Matching services"
      class={cn(
        'py-1',
        inline
          ? 'mt-2'
          : 'absolute inset-x-0 top-full z-50 mt-1 max-h-80 overflow-y-auto rounded-xl border bg-popover text-popover-foreground shadow-overlay'
      )}
    >
      {#each results as result, idx (result.item.id)}
        <!-- Keyboard selection is handled on the combobox input. -->
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <li
          id={`${listId}-${idx}`}
          role="option"
          aria-selected={idx === highlighted}
          class={cn(
            'mx-1 flex cursor-pointer flex-col rounded-lg px-2.5 py-1.5',
            idx === highlighted && 'bg-raised'
          )}
          onmousedown={(e) => e.preventDefault()}
          onclick={() => select(result.item.id)}
          onmouseenter={() => (highlighted = idx)}
        >
          <span class="truncate text-sm font-medium">{result.item.name}</span>
          {#if result.item.target}
            <span class="truncate font-mono text-xs text-muted-foreground">
              {result.item.target}
            </span>
          {/if}
        </li>
      {:else}
        <li class="px-3.5 py-2 text-sm text-muted-foreground">No services match “{search}”.</li>
      {/each}
    </ul>
  {/if}
</div>
