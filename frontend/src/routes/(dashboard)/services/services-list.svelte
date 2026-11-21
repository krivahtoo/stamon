<script>
  import { untrack } from 'svelte';
  import Columns3 from '@lucide/svelte/icons/columns-3';
  import Search from '@lucide/svelte/icons/search';
  import Server from '@lucide/svelte/icons/server';

  import { goto } from '$app/navigation';
  import services, { servicesLoaded } from '$lib/store/services.js';
  import { loadRecentChecks } from '$lib/data/checks.js';
  import { statuses, StatusValue } from '$lib/data/table.js';
  import { cn } from '$lib/utils.js';
  import * as Card from '$lib/components/ui/card/index.js';
  import * as Table from '$lib/components/ui/table/index.js';
  import * as Select from '$lib/components/ui/select/index.js';
  import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
  import { Badge } from '$lib/components/ui/badge/index.js';
  import { Button } from '$lib/components/ui/button/index.js';
  import { Input } from '$lib/components/ui/input/index.js';
  import { EmptyState, StatusDot, StatusPill, UptimeBar } from '$lib/components/dashboard/index.js';
  import { formatDuration } from '../(components)/format.js';
  import RowActions from './(components)/row-actions.svelte';

  /**
   * @typedef {import('$lib/store/services').Service} Service
   * @typedef {'all' | 'up' | 'down' | 'pending' | 'paused'} StatusFilter
   */

  const PAGE_SIZE = 10;

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

  /** @type {{ value: StatusFilter, label: string, tone?: import('$lib/components/dashboard/tones.js').Tone }[]} */
  const chips = [
    { value: 'all', label: 'All' },
    { value: 'up', label: 'Up', tone: 'up' },
    { value: 'down', label: 'Down', tone: 'down' },
    { value: 'pending', label: 'Pending', tone: 'pending' },
    { value: 'paused', label: 'Paused', tone: 'neutral' }
  ];

  /** @type {{ key: 'type' | 'tags' | 'uptime' | 'lastCheck', label: string }[]} */
  const columns = [
    { key: 'type', label: 'Type' },
    { key: 'tags', label: 'Tags' },
    { key: 'uptime', label: 'Uptime' },
    { key: 'lastCheck', label: 'Last check' }
  ];

  /**
   * The status chip a service falls under. Maintenance only shows under All.
   * @param {Service} service
   * @returns {StatusFilter | undefined}
   */
  function chipOf(service) {
    if (!service.active) return 'paused';
    switch (service.last_status) {
      case StatusValue.Up:
        return 'up';
      case StatusValue.Down:
      case StatusValue.Failed:
        return 'down';
      case StatusValue.Pending:
        return 'pending';
    }
  }

  /** @param {Service} service */
  const toneOf = (service) =>
    service.active
      ? (statuses.find((s) => s.value === service.last_status)?.tone ?? 'neutral')
      : 'neutral';

  /** @param {string} type */
  const typeLabel = (type) => typeLabels[type] ?? type;

  /** @param {Service} service */
  const targetOf = (service) => service.target ?? 'push';

  /** @param {string} [time] */
  const checkedAgo = (time) =>
    time ? `${formatDuration(time, new Date().toISOString())} ago` : '—';

  /** @type {StatusFilter} */
  let status = $state('all');
  let query = $state('');
  let type = $state('all');
  let tag = $state('all');
  let page = $state(0);
  let visible = $state({ type: true, tags: true, uptime: true, lastCheck: true });

  let counts = $derived(
    $services.reduce(
      (counts, service) => {
        const chip = chipOf(service);
        if (chip) counts[chip]++;
        return counts;
      },
      { all: $services.length, up: 0, down: 0, pending: 0, paused: 0 }
    )
  );

  let types = $derived(
    [...new Set($services.map((service) => service.service_type))].sort((a, b) =>
      typeLabel(a).localeCompare(typeLabel(b))
    )
  );

  let tags = $derived([...new Set($services.flatMap((service) => service.tags ?? []))].sort());

  let filtered = $derived.by(() => {
    const q = query.trim().toLowerCase();
    return $services.filter(
      (service) =>
        (status === 'all' || chipOf(service) === status) &&
        (type === 'all' || service.service_type === type) &&
        (tag === 'all' || (service.tags ?? []).includes(tag)) &&
        (!q ||
          [service.name, service.target ?? '', ...(service.tags ?? [])].some((value) =>
            value.toLowerCase().includes(q)
          ))
    );
  });

  let pageCount = $derived(Math.max(1, Math.ceil(filtered.length / PAGE_SIZE)));
  // Stay on a real page when services are removed or filtered out.
  let current = $derived(Math.min(page, pageCount - 1));
  let shown = $derived(filtered.slice(current * PAGE_SIZE, (current + 1) * PAGE_SIZE));
  let range = $derived(
    filtered.length
      ? `${current * PAGE_SIZE + 1}–${current * PAGE_SIZE + shown.length} of ${filtered.length}`
      : '0 of 0'
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

  function clearFilters() {
    status = 'all';
    query = '';
    type = 'all';
    tag = 'all';
    page = 0;
  }

  /**
   * Open the service when its row is clicked, unless a link or button in it was.
   * @param {Event} event
   * @param {Service} service
   */
  function openRow(event, service) {
    const target = /** @type {Element} */ (event.target);
    if (target.closest('a, button, [role="menuitem"]')) return;
    goto(`/services/${service.id}`);
  }
</script>

<div class="flex flex-col gap-4">
  <!-- The filters move to their own line when they don't fit beside the chips. -->
  <div class="flex flex-wrap items-center justify-between gap-3">
    <div class="flex flex-wrap gap-2" role="group" aria-label="Filter by status">
      {#each chips as chip (chip.value)}
        <button
          type="button"
          aria-pressed={status === chip.value}
          onclick={() => {
            status = chip.value;
            page = 0;
          }}
          class={cn(
            'inline-flex h-8 items-center gap-1.5 rounded-full border bg-background px-3 text-sm font-medium transition-colors duration-160 ease-in-out hover:bg-surface',
            status === chip.value && 'border-primary bg-primary/5 text-primary'
          )}
        >
          {#if chip.tone}
            <StatusDot tone={chip.tone} size="sm" />
          {/if}
          {chip.label}
          <span class="font-mono text-xs text-muted-foreground tabular-nums">
            {counts[chip.value]}
          </span>
        </button>
      {/each}
    </div>

    <div class="grid w-full grid-cols-2 gap-2 sm:flex sm:w-auto sm:items-center">
      <div class="relative col-span-2 sm:w-72">
        <Search
          class="pointer-events-none absolute top-1/2 left-3 size-4 -translate-y-1/2 text-muted-foreground"
          aria-hidden="true"
        />
        <Input
          type="search"
          placeholder="Filter by name, target or tag"
          aria-label="Filter services"
          class="pl-9"
          bind:value={query}
          oninput={() => (page = 0)}
        />
      </div>
      <Select.Root type="single" bind:value={type} onValueChange={() => (page = 0)}>
        <Select.Trigger aria-label="Filter by type" class="h-10 w-full sm:w-36">
          {type === 'all' ? 'All types' : typeLabel(type)}
        </Select.Trigger>
        <Select.Content>
          <Select.Item value="all" label="All types" />
          {#each types as value (value)}
            <Select.Item {value} label={typeLabel(value)} />
          {/each}
        </Select.Content>
      </Select.Root>
      <Select.Root type="single" bind:value={tag} onValueChange={() => (page = 0)}>
        <Select.Trigger aria-label="Filter by tag" class="h-10 w-full sm:w-36">
          <span class="truncate">{tag === 'all' ? 'All tags' : tag}</span>
        </Select.Trigger>
        <Select.Content>
          <Select.Item value="all" label="All tags" />
          {#each tags as value (value)}
            <Select.Item {value} label={value} />
          {/each}
        </Select.Content>
      </Select.Root>
      <!-- Rows are cards on mobile, so there are no columns to hide. -->
      <DropdownMenu.Root>
        <DropdownMenu.Trigger>
          {#snippet child({ props })}
            <Button {...props} variant="outline" class="hidden h-10 sm:inline-flex">
              <Columns3 />
              Columns
            </Button>
          {/snippet}
        </DropdownMenu.Trigger>
        <DropdownMenu.Content align="end">
          {#each columns as column (column.key)}
            <DropdownMenu.CheckboxItem bind:checked={visible[column.key]}>
              {column.label}
            </DropdownMenu.CheckboxItem>
          {/each}
        </DropdownMenu.Content>
      </DropdownMenu.Root>
    </div>
  </div>

  {#if $servicesLoaded && $services.length === 0}
    <EmptyState icon={Server} message="No services yet. Add one to start monitoring.">
      <Button href="/services/new">New service</Button>
    </EmptyState>
  {:else}
    <Card.Root class="gap-0 py-0">
      {#if !$servicesLoaded}
        <p class="px-5 py-8 text-center text-sm text-muted-foreground">Loading services…</p>
      {:else if filtered.length === 0}
        <p class="px-5 py-8 text-center text-sm text-muted-foreground">
          No services match these filters.
          <Button variant="link" class="h-auto px-1" onclick={clearFilters}>Clear filters</Button>
        </p>
      {:else}
        <!-- Table from sm up -->
        <div class="hidden sm:block">
          <Table.Root>
            <Table.Header>
              <Table.Row class="bg-surface hover:bg-surface">
                <Table.Head>Service</Table.Head>
                {#if visible.type}<Table.Head>Type</Table.Head>{/if}
                {#if visible.tags}<Table.Head>Tags</Table.Head>{/if}
                {#if visible.uptime}
                  <Table.Head class="w-full">Uptime · recent checks</Table.Head>
                {/if}
                {#if visible.lastCheck}<Table.Head class="text-right">Last check</Table.Head>{/if}
                <Table.Head><span class="sr-only">Actions</span></Table.Head>
              </Table.Row>
            </Table.Header>
            <Table.Body>
              {#each shown as service (service.id)}
                {@const recent = checks[service.id]}
                <Table.Row
                  class="cursor-pointer last:border-b-0"
                  onclick={(/** @type {Event} */ e) => openRow(e, service)}
                >
                  <Table.Cell class="h-14">
                    <div class="flex items-center gap-2.5">
                      <StatusDot tone={toneOf(service)} />
                      <div class="flex min-w-0 flex-col">
                        <a
                          href="/services/{service.id}"
                          class={cn(
                            'font-semibold underline-offset-4 hover:text-primary hover:underline',
                            !service.active && 'text-muted-foreground'
                          )}
                        >
                          {service.name}
                        </a>
                        <span class="max-w-56 truncate font-mono text-xs text-muted-foreground">
                          {targetOf(service)}
                        </span>
                      </div>
                    </div>
                  </Table.Cell>
                  {#if visible.type}
                    <Table.Cell>
                      <Badge variant="secondary">{typeLabel(service.service_type)}</Badge>
                    </Table.Cell>
                  {/if}
                  {#if visible.tags}
                    <Table.Cell>
                      <div class="flex flex-wrap gap-1">
                        {#each service.tags ?? [] as value (value)}
                          <Badge variant="outline" class="font-normal">{value}</Badge>
                        {/each}
                      </div>
                    </Table.Cell>
                  {/if}
                  {#if visible.uptime}
                    <Table.Cell class="max-w-0">
                      {#if service.active}
                        <UptimeBar ticks={recent?.ticks ?? []} uptime={recent?.uptime} />
                      {:else}
                        <div class="flex items-center gap-3">
                          <UptimeBar ticks={recent?.ticks ?? []} class="opacity-40" />
                          <StatusPill>Paused</StatusPill>
                        </div>
                      {/if}
                    </Table.Cell>
                  {/if}
                  {#if visible.lastCheck}
                    <Table.Cell class="text-right text-muted-foreground tabular-nums">
                      {service.active ? checkedAgo(recent?.lastCheck) : '—'}
                    </Table.Cell>
                  {/if}
                  <Table.Cell class="w-0">
                    <RowActions {service} />
                  </Table.Cell>
                </Table.Row>
              {/each}
            </Table.Body>
          </Table.Root>
        </div>

        <!-- Cards on mobile -->
        <ul class="divide-y sm:hidden">
          {#each shown as service (service.id)}
            {@const recent = checks[service.id]}
            <li class="flex flex-col gap-2 px-4 py-3">
              <div class="flex items-start justify-between gap-2">
                <div class="flex min-w-0 items-start gap-2.5">
                  <StatusDot tone={toneOf(service)} class="mt-1.5" />
                  <div class="flex min-w-0 flex-col">
                    <a href="/services/{service.id}" class="font-semibold">{service.name}</a>
                    <span class="truncate font-mono text-xs text-muted-foreground">
                      {targetOf(service)}
                    </span>
                  </div>
                </div>
                <div class="flex shrink-0 items-center gap-1">
                  {#if !service.active}
                    <StatusPill>Paused</StatusPill>
                  {:else if recent?.uptime !== undefined}
                    <span class="font-mono text-sm font-bold tabular-nums">
                      {recent.uptime.toFixed(2)}%
                    </span>
                  {/if}
                  <RowActions {service} />
                </div>
              </div>
              <UptimeBar ticks={recent?.ticks ?? []} class={cn(!service.active && 'opacity-40')} />
              <div class="flex items-center justify-between gap-2">
                <p class="text-xs text-muted-foreground tabular-nums">
                  {typeLabel(service.service_type)}
                  {#if !service.active}
                    · Paused
                  {:else}
                    {#if recent?.latency !== undefined}· {recent.latency} ms{/if}
                    · checked {checkedAgo(recent?.lastCheck)}
                  {/if}
                </p>
                <div class="flex flex-wrap justify-end gap-1">
                  {#each service.tags ?? [] as value (value)}
                    <Badge variant="outline" class="font-normal">{value}</Badge>
                  {/each}
                </div>
              </div>
            </li>
          {/each}
        </ul>

        <div class="flex items-center justify-between gap-2 border-t px-4 py-3 sm:px-5">
          <p class="text-sm text-muted-foreground tabular-nums">
            <span class="hidden sm:inline">Showing</span>
            {range}
          </p>
          <div class="flex gap-2">
            <Button
              variant="outline"
              size="sm"
              disabled={current === 0}
              onclick={() => (page = current - 1)}
            >
              Previous
            </Button>
            <Button
              variant="outline"
              size="sm"
              disabled={current >= pageCount - 1}
              onclick={() => (page = current + 1)}
            >
              Next
            </Button>
          </div>
        </div>
      {/if}
    </Card.Root>
  {/if}
</div>
