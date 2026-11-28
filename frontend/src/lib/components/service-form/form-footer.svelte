<script>
  import { Button } from '$lib/components/ui/button/index.js';

  /**
   * The sticky Cancel/Save footer of the new and edit service pages. It sits
   * in the page flow after the form, so it never covers the last fields.
   * @typedef {Object} Props
   * @property {string} form - The id of the form it submits.
   * @property {string} note - One muted line on the left.
   * @property {string} submitLabel
   * @property {boolean} [disabled]
   * @property {() => void} oncancel
   */

  /** @type {Props} */
  let { form, note, submitLabel, disabled = false, oncancel } = $props();
</script>

<footer class="sticky bottom-0 -mx-4 -mb-8 border-t bg-background px-4 py-3 md:-mx-8 md:px-8">
  <div class="mx-auto flex max-w-3xl items-center gap-2">
    <p class="mr-auto hidden text-sm text-muted-foreground sm:block">{note}</p>
    <Button variant="outline" class="flex-1 sm:flex-none" onclick={oncancel}>Cancel</Button>
    <Button type="submit" {form} class="flex-1 sm:flex-none" {disabled}>{submitLabel}</Button>
  </div>
</footer>
