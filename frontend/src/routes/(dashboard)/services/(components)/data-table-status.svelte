<script>
  import { Checkbox } from '$lib/components/ui/checkbox/index.js';
  import { writable } from 'svelte/store';

  /**
   * @typedef {import("svelte/store").Writable<boolean>} Writable
   * @typedef {import("svelte/elements").HTMLButtonAttributes} HTMLButtonAttributes
   */

  /**
   * @typedef {HTMLButtonAttributes & { checked?: Writable }} Props
   */

  /** @type {Props & { [key: string]: any }} */
  let { checked = writable(false), ...rest } = $props();
</script>

<Checkbox bind:checked={$checked} {...rest} />
