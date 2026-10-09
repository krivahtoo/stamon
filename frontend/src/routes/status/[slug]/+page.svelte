<script>
  import { onDestroy, onMount } from 'svelte';
  import { invalidateAll } from '$app/navigation';

  import { Badge } from '$lib/components/ui/badge/index.js';
  import * as Card from '$lib/components/ui/card/index.js';
  import * as Tooltip from '$lib/components/ui/tooltip/index.js';
  import { cn } from '$lib/utils.js';
  import { statuses } from '$lib/data/table.js';

  /** @type {import('./$types').PageData} */
  export let data;
  $: page = data.page;

  /** @type {Record<string, { label: string, class: string }>} */
  const overall = {
    operational: { label: 'All systems operational', class: 'bg-green-600 text-white' },
    degraded: { label: 'Some systems are down', class: 'bg-yellow-500 text-black' },
    outage: { label: 'Major outage', class: 'bg-red-600 text-white' },
    maintenance: { label: 'Maintenance in progress', class: 'bg-blue-600 text-white' }
  };

  /** @param {number | null} uptime */
  function barClass(uptime) {
    if (uptime === null) return 'bg-muted';
    if (uptime >= 99.5) return 'bg-green-500 dark:bg-green-500/80';
    if (uptime >= 95) return 'bg-yellow-500';
    return 'bg-red-500 dark:bg-red-500/80';
  }

  /** @param {number | null} uptime */
  const formatUptime = (uptime) =>
    uptime === null ? 'No data' : `${uptime.toFixed(uptime === 100 ? 0 : 2)}%`;

  /** @param {string} time */
  const formatTime = (time) => new Date(time).toLocaleString();

  /** @param {string} date */
  const formatDate = (date) =>
    new Date(`${date}T00:00:00Z`).toLocaleDateString(undefined, { timeZone: 'UTC' });

  // Keep the page current for visitors who leave it open.
  /** @type {ReturnType<typeof setInterval> | undefined} */
  let refresh;
  onMount(() => {
    refresh = setInterval(() => invalidateAll(), 60000);
  });
  onDestroy(() => clearInterval(refresh));
</script>

<svelte:head>
  <title>{page.title}</title>
</svelte:head>

<main class="mx-auto w-full max-w-3xl space-y-6 px-4 py-10">
  <header class="space-y-1">
    <h1 class="text-3xl font-bold tracking-tight">{page.title}</h1>
    {#if page.description}
      <p class="text-muted-foreground">{page.description}</p>
    {/if}
  </header>

  <div class={cn('rounded-lg px-4 py-3 font-medium', overall[page.status]?.class)}>
    {overall[page.status]?.label ?? page.status}
  </div>

  {#each page.maintenance as window}
    <Card.Root class="border-blue-500/50">
      <Card.Header class="pb-2">
        <Card.Title class="text-base">{window.title}</Card.Title>
        <Card.Description>
          Scheduled maintenance: {formatTime(window.starts_at)} – {formatTime(window.ends_at)}
        </Card.Description>
      </Card.Header>
      {#if window.description}
        <Card.Content class="text-sm">{window.description}</Card.Content>
      {/if}
    </Card.Root>
  {/each}

  <Card.Root>
    <Card.Content class="divide-y p-0">
      {#each page.services as service}
        {@const status = statuses.find((s) => s.value === service.status)}
        <section class="space-y-2 p-4">
          <div class="flex items-center justify-between gap-2">
            <h2 class="font-medium">{service.name}</h2>
            <div class="flex items-center gap-2 text-sm text-muted-foreground">
              <span>{formatUptime(service.uptime)}</span>
              <Badge variant={status?.variant}>{status?.label ?? 'Unknown'}</Badge>
            </div>
          </div>
          <div class="flex h-8 gap-[2px]" aria-label={`Daily uptime of ${service.name}`}>
            {#each service.days as day}
              <Tooltip.Root openDelay={0}>
                <Tooltip.Trigger
                  class={cn('h-full flex-1 rounded-sm', barClass(day.uptime))}
                  aria-label={`${formatDate(day.date)}: ${formatUptime(day.uptime)}`}
                />
                <Tooltip.Content>
                  {formatDate(day.date)}: {formatUptime(day.uptime)}
                </Tooltip.Content>
              </Tooltip.Root>
            {/each}
          </div>
          <div class="flex justify-between text-xs text-muted-foreground">
            <span>{service.days.length} days ago</span>
            <span>Today</span>
          </div>
        </section>
      {:else}
        <p class="p-4 text-sm text-muted-foreground">No services on this page yet.</p>
      {/each}
    </Card.Content>
  </Card.Root>

  <footer class="text-center text-xs text-muted-foreground">
    Updated {formatTime(page.generated_at)} · Powered by Stamon
  </footer>
</main>
