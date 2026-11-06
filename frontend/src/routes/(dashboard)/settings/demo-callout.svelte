<script>
  import demo from '$lib/store/demo.js';
  import { Callout } from '$lib/components/dashboard/index.js';
</script>

{#if $demo}
  <Callout tone="pending" class="mb-6">Settings can't be changed in the public demo.</Callout>
{/if}
