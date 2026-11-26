<script>
  import ServiceDetail from './(components)/service-detail.svelte';

  /**
   * @typedef {Object} Props
   * @property {import('./$types').PageData} data
   */

  /** @type {Props} */
  let { data } = $props();
</script>

<!-- Keyed so moving to another service resets the page's state. -->
{#key data.service.id}
  <ServiceDetail service={data.service} />
{/key}
