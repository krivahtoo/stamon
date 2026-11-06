<script>
  import demo from '$lib/store/demo.js';
</script>

{#if $demo}
  <p
    class="mb-6 rounded-md border-l-[3px] border-status-pending bg-status-pending-tint px-4 py-3 text-sm text-muted-foreground"
  >
    Settings can't be changed in the public demo.
  </p>
{/if}
