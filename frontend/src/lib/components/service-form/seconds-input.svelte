<script>
  import { Input } from '$lib/components/ui/input/index.js';

  /**
   * A number input with a "seconds" suffix.
   * @typedef {Object} Props
   * @property {number | string} [value]
   * @property {string} [id]
   * @property {string} [min]
   * @property {string} [max]
   * @property {boolean} [required]
   */

  /** @type {Props} */
  let { value = $bindable(), id, min, max, required = false } = $props();
</script>

<div class="relative">
  <Input {id} type="number" bind:value {min} {max} {required} class="pr-18" />
  <span
    class="pointer-events-none absolute inset-y-0 right-3 flex items-center text-sm text-muted-foreground"
  >
    seconds
  </span>
</div>
