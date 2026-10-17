<script>
  import demo from '$lib/store/demo.js';
</script>

{#if $demo}
  <p
    class="mb-6 rounded-md border-l-[3px] border-amber-500 bg-amber-500/10 px-4 py-3 text-sm text-muted-foreground"
  >
    Settings can't be changed in the public demo.
  </p>
{/if}
