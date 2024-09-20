<script>
  import { cubicInOut } from 'svelte/easing';
  import { crossfade } from 'svelte/transition';
  import { cn } from '$lib/utils.js';
  import { page } from '$app/stores';
  import { Button } from '$lib/components/ui/button/index.js';


  /**
   * @type {string | undefined | null}
   */
  let className = undefined;

  /**
   * @typedef {Object} Item
   * @property {string} href
   * @property {string} title
   */
  /**
   * @type {Item[]}
   */
  export let items;
  /**
   * @type {string | undefined | null}
   */
  export { className as class };

  const [send, receive] = crossfade({
    duration: 250,
    easing: cubicInOut
  });
</script>

<nav class={cn('flex space-x-2 md:flex-col md:space-x-0 lg:space-y-1', className)}>
  {#each items as item}
    {@const isActive = $page.url.pathname === item.href}

    <Button
      href={item.href}
      variant="ghost"
      class={cn(!isActive && 'hover:text-primary', 'relative justify-start hover:bg-transparent')}
      data-sveltekit-noscroll
      data-sveltekit-replacestate
    >
      {#if isActive}
        <div
          class="absolute inset-0 rounded-md bg-muted"
          in:send={{ key: 'active-sidebar-tab' }}
          out:receive={{ key: 'active-sidebar-tab' }}
        />
      {/if}
      <div class="relative">
        {item.title}
      </div>
    </Button>
  {/each}
</nav>