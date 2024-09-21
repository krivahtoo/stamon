<script>
  import { fly } from 'svelte/transition';
  import DataTable from './data-table.svelte';
</script>

<div in:fly={{ y: 20, duration: 200 }} class="flex items-center justify-between space-y-2">
  <div>
    <h2 class="text-2xl font-bold tracking-tight">Services</h2>
    <p class="text-muted-foreground">Here's a list of your services monitored!</p>
  </div>
</div>

<DataTable />
