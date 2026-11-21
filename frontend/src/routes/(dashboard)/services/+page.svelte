<script>
  import Plus from '@lucide/svelte/icons/plus';
  import services, { servicesLoaded } from '$lib/store/services.js';
  import { Button } from '$lib/components/ui/button/index.js';
  import { PageHeader } from '$lib/components/dashboard/index.js';
  import ServicesList from './services-list.svelte';
</script>

<div class="flex flex-col gap-6">
  <PageHeader
    label="Infrastructure"
    title="Services"
    count={$servicesLoaded ? $services.length : undefined}
    description="Everything Stamon checks, and how each check is doing."
  >
    {#snippet actions()}
      <Button href="/services/new">
        <Plus />
        New service
      </Button>
    {/snippet}
  </PageHeader>

  <ServicesList />
</div>
