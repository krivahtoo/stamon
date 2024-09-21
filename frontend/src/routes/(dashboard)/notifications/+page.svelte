<script>
  import { Button } from '$lib/components/ui/button/index.js';
</script>

<section class="h-full">
  <div class="flex flex-1 h-full items-center justify-center rounded-lg border border-dashed shadow-sm">
    <div class="flex flex-col items-center gap-1 text-center">
      <h3 class="text-2xl font-bold tracking-tight">You have no Notification Providers</h3>
      <p class="text-sm text-muted-foreground">
        Add a Notifiction ptovider to start receiving status notifictions
      </p>
      <Button class="mt-4">Add Provider</Button>
    </div>
  </div>
</section>