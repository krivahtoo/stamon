<script>
  import { untrack } from 'svelte';
  import ArrowRight from '@lucide/svelte/icons/arrow-right';
  import Server from '@lucide/svelte/icons/server';

  import services, { servicesLoaded } from '$lib/store/services.js';
  import { loadRecentChecks } from '$lib/data/checks.js';
  import { statuses, StatusValue } from '$lib/data/table.js';
  import * as Card from '$lib/components/ui/card/index.js';
  import * as Table from '$lib/components/ui/table/index.js';
  import { Badge } from '$lib/components/ui/badge/index.js';
  import { Button } from '$lib/components/ui/button/index.js';
  import { EmptyState, StatusDot, UptimeBar } from '$lib/components/dashboard/index.js';
  import { formatDuration } from './format.js';

  const LIMIT = 6;

  // Services needing attention come first.
  const priority = [
    StatusValue.Down,
    StatusValue.Failed,
    StatusValue.Pending,
    StatusValue.Maintenance,
    StatusValue.Up
  ];

  /** @type {Record<string, string>} */
  const typeLabels = {
    http: 'HTTP',
    ping: 'Ping',
    tcp: 'TCP',
    dns: 'DNS',
    tls: 'TLS',
    docker: 'Docker',
    database: 'Database',
    push: 'Push'
  };

  let active = $derived($services.filter((service) => service.active));

  let shown = $derived(
    [...active]
      .sort(
        (a, b) =>
          priority.indexOf(a.last_status) - priority.indexOf(b.last_status) ||
          a.name.localeCompare(b.name)
      )
      .slice(0, LIMIT)
  );

  /** @type {Record<number, import('$lib/data/checks.js').RecentChecks>} */
  let checks = $state({});

  // Load the recent checks of each shown service once.
  $effect(() => {
    const ids = shown.map((service) => service.id);
    untrack(() => {
      for (const id of ids) {
        if (id in checks) continue;
        checks[id] = { ticks: [] };
        loadRecentChecks(id).then((result) => (checks[id] = result));
      }
    });
  });

  /** @param {number} status */
  const toneOf = (status) => statuses.find((s) => s.value === status)?.tone ?? 'neutral';

  /** @param {string} [time] */
  const checkedAgo = (time) =>
    time ? `${formatDuration(time, new Date().toISOString())} ago` : '–';

  /** @param {number} [ms] */
  const formatLatency = (ms) => (ms === undefined ? '–' : `${ms} ms`);
</script>

<Card.Root class="gap-0 py-0">
  <div class="flex items-center gap-2.5 border-b px-4 py-4 sm:px-5">
    <h2 class="text-lg font-bold tracking-[-0.015em]">Services</h2>
    {#if active.length}
      <span
        class="rounded-full bg-raised px-2 py-px text-xs font-semibold text-muted-foreground tabular-nums"
      >
        {active.length}
      </span>
      <p class="hidden text-sm text-muted-foreground sm:block">Needing attention first</p>
    {/if}
    <Button variant="link" href="/services" class="ml-auto px-0">
      View all
      <ArrowRight />
    </Button>
  </div>

  {#if $servicesLoaded && active.length === 0}
    <EmptyState
      icon={Server}
      message="No services yet. Add one to start monitoring."
      framed={false}
    >
      <Button href="/services/new">New service</Button>
    </EmptyState>
  {:else}
    <!-- Table from sm up -->
    <div class="hidden sm:block">
      <Table.Root>
        <Table.Header>
          <Table.Row class="bg-surface hover:bg-surface">
            <Table.Head>Service</Table.Head>
            <Table.Head>Type</Table.Head>
            <Table.Head class="w-full">Recent checks</Table.Head>
            <Table.Head class="hidden text-right lg:table-cell">Latency</Table.Head>
            <Table.Head class="text-right">Last check</Table.Head>
          </Table.Row>
        </Table.Header>
        <Table.Body>
          {#each shown as service (service.id)}
            {@const recent = checks[service.id]}
            <Table.Row class="last:border-b-0">
              <Table.Cell class="h-14">
                <div class="flex items-center gap-2.5">
                  <StatusDot tone={toneOf(service.last_status)} />
                  <div class="flex min-w-0 flex-col">
                    <a
                      href="/services/{service.id}"
                      class="font-semibold underline-offset-4 hover:text-primary hover:underline"
                    >
                      {service.name}
                    </a>
                    <span class="max-w-56 truncate font-mono text-xs text-muted-foreground">
                      {service.target ?? 'push'}
                    </span>
                  </div>
                </div>
              </Table.Cell>
              <Table.Cell>
                <Badge variant="secondary">
                  {typeLabels[service.service_type] ?? service.service_type}
                </Badge>
              </Table.Cell>
              <Table.Cell class="max-w-0">
                <UptimeBar ticks={recent?.ticks ?? []} uptime={recent?.uptime} />
              </Table.Cell>
              <Table.Cell
                class="hidden text-right font-mono text-muted-foreground tabular-nums lg:table-cell"
              >
                {formatLatency(recent?.latency)}
              </Table.Cell>
              <Table.Cell class="text-right text-muted-foreground tabular-nums">
                {checkedAgo(recent?.lastCheck)}
              </Table.Cell>
            </Table.Row>
          {:else}
            <Table.Row class="hover:bg-transparent">
              <Table.Cell colspan={5} class="h-24 text-center text-muted-foreground">
                Loading services…
              </Table.Cell>
            </Table.Row>
          {/each}
        </Table.Body>
      </Table.Root>
    </div>

    <!-- Stacked list on mobile -->
    <ul class="divide-y sm:hidden">
      {#each shown as service (service.id)}
        {@const recent = checks[service.id]}
        <li class="flex flex-col gap-2 px-4 py-3">
          <div class="flex items-start justify-between gap-2">
            <div class="flex min-w-0 items-start gap-2.5">
              <StatusDot tone={toneOf(service.last_status)} class="mt-1.5" />
              <div class="flex min-w-0 flex-col">
                <a href="/services/{service.id}" class="font-semibold">{service.name}</a>
                <span class="truncate font-mono text-xs text-muted-foreground">
                  {service.target ?? 'push'}
                </span>
              </div>
            </div>
            {#if recent?.uptime !== undefined}
              <span class="shrink-0 font-mono text-sm font-bold tabular-nums">
                {recent.uptime.toFixed(2)}%
              </span>
            {/if}
          </div>
          <UptimeBar ticks={recent?.ticks ?? []} />
          <p class="text-xs text-muted-foreground tabular-nums">
            {typeLabels[service.service_type] ?? service.service_type}
            {#if recent?.latency !== undefined}· {formatLatency(recent.latency)}{/if}
            · checked {checkedAgo(recent?.lastCheck)}
          </p>
        </li>
      {:else}
        <li class="px-4 py-8 text-center text-sm text-muted-foreground">Loading services…</li>
      {/each}
    </ul>
  {/if}
</Card.Root>
