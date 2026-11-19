<script>
  /**
   * @typedef {Object} Props
   * @property {number} id
   * @property {string} name
   */

  /** @type {Props} */
  let { id, name } = $props();
</script>

<a
  href="/services/{id}"
  class="hover:text-primary font-semibold underline-offset-4 hover:underline"
>
  {name}
</a>
