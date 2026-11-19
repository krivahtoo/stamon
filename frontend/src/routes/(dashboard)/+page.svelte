<script>
  import { writable } from 'svelte/store';
  import { onMount } from 'svelte';
  import { fly } from 'svelte/transition';
  import { Render, Subscribe, createRender, createTable } from 'svelte-headless-table';
  import { addColumnFilters, addPagination, addTableFilter } from 'svelte-headless-table/plugins';

  import stats from '$lib/store/stats.js';
  import { cfetch } from '$lib/utils.js';

  import * as Table from '$lib/components/ui/table/index.js';
  import * as Card from '$lib/components/ui/card/index.js';
  import { Button } from '$lib/components/ui/button/index.js';
  import { Input } from '$lib/components/ui/input/index.js';
  import { PageHeader, StatCard, EmptyState } from '$lib/components/dashboard/index.js';
  import StatusCell from '$lib/components/table/status-cell.svelte';
  import Plus from '@lucide/svelte/icons/plus';
  import Wrench from '@lucide/svelte/icons/wrench';
  import Search from '@lucide/svelte/icons/search';
  import CircleCheck from '@lucide/svelte/icons/circle-check';
  import ChevronLeft from '@lucide/svelte/icons/chevron-left';
  import ChevronRight from '@lucide/svelte/icons/chevron-right';

  import ServiceCell from './(components)/service-cell.svelte';
  import MessageCell from './(components)/message-cell.svelte';
  import DurationCell from './(components)/duration-cell.svelte';
  import DateCell from './(components)/date-cell.svelte';
  import DateFilter from './(components)/date-filter.svelte';
  import { formatDuration, formatStarted } from './(components)/format.js';
  import { goto } from '$app/navigation';
  import { toast } from 'svelte-sonner';

  /**
   * @typedef {Object} Incident
   * @property {number} service_id - The service id.
   * @property {string} service_name - The service name.
   * @property {string} service_target - The URL or host of the service.
   * @property {number} status - The incident log status.
   * @property {string} date - The date of the incident.
   * @property {string} messages - The error messages.
   * @property {number} count - Number of checks.
   * @property {string} start - The start of the incident.
   * @property {string} end - The end of the incident.
   */

  /** @type {import('svelte/store').Writable<Incident[]>} */
  const data = writable([]);
  let loaded = $state(false);

  const table = createTable(data, {
    page: addPagination({ initialPageSize: 10 }),
    filter: addTableFilter({
      fn: ({ filterValue, value }) => {
        return value.toLowerCase().includes(filterValue.toLowerCase());
      }
    }),
    colFilter: addColumnFilters()
  });

  // Search only looks at the service name and the message.
  const noSearch = { filter: { exclude: true } };

  const columns = table.createColumns([
    table.column({
      header: 'Service',
      accessor: 'service_name',
      id: 'name',
      cell: ({ value, row }) => {
        if (row.isData()) {
          return createRender(/** @type {any} */ (ServiceCell), {
            id: row.original.service_id,
            name: value
          });
        }
        return value;
      }
    }),
    table.column({
      header: 'Status',
      accessor: 'status',
      id: 'status',
      cell: ({ value }) => createRender(/** @type {any} */ (StatusCell), { statusId: value }),
      plugins: noSearch
    }),
    table.column({
      header: 'Message',
      accessor: 'messages',
      id: 'message',
      cell: ({ value }) => createRender(/** @type {any} */ (MessageCell), { message: value })
    }),
    table.column({
      header: 'Duration',
      accessor: 'start',
      id: 'duration',
      cell: ({ row }) => {
        if (row.isData()) {
          return createRender(/** @type {any} */ (DurationCell), {
            start: row.original.start,
            end: row.original.end
          });
        }
        return '';
      },
      plugins: noSearch
    }),
    table.column({
      header: 'Checks',
      accessor: 'count',
      id: 'count',
      plugins: noSearch
    }),
    table.column({
      header: 'Started',
      accessor: 'date',
      id: 'date',
      cell: ({ row }) => {
        if (row.isData()) {
          return createRender(/** @type {any} */ (DateCell), { start: row.original.start });
        }
        return '';
      },
      plugins: {
        ...noSearch,
        colFilter: {
          fn: ({ filterValue, value }) => {
            if (!Array.isArray(filterValue) || filterValue.length === 0) return true;
            return filterValue.includes(value);
          },
          initialFilterValue: []
        }
      }
    })
  ]);

  const tableModel = table.createViewModel(columns);

  const { headerRows, pageRows, rows, tableAttrs, tableBodyAttrs, pluginStates } = tableModel;
  const { filterValue } = pluginStates.filter;
  const { filterValues } = pluginStates.colFilter;
  const { pageIndex, pageCount, hasNextPage, hasPreviousPage } = pluginStates.page;

  // Right-align the number and date columns, as in the design.
  /** @param {string} id */
  const alignEnd = (id) => (id === 'count' || id === 'date' ? 'text-right' : '');

  // The message takes the width left over and truncates, so the table never scrolls.
  /** @param {string} id */
  const cellClass = (id) => (id === 'message' ? 'w-full max-w-0' : alignEnd(id));

  let days = $derived([...new Set($data.map((incident) => incident.date))]);

  /** @type {string[]} */
  let selectedDays = $state([]);
  $effect(() => {
    filterValues.update((values) => ({ ...values, date: selectedDays }));
  });

  // Go back to the first page whenever the filters change.
  $effect(() => {
    void $filterValue;
    void selectedDays.length;
    pageIndex.set(0);
  });

  let paused = $derived(Math.max($stats.count - $stats.active, 0));

  onMount(() => {
    cfetch('/logs/incidents')
      .then(async (res) => {
        if (res.ok) {
          const json_data = await res.json();
          data.set(json_data.incidents);
        } else {
          if (res.status === 401) {
            goto('/login');
          }
        }
      })
      .catch((e) => {
        toast.error('Failed to get incidents', {
          description: `${e}`
        });
      })
      .finally(() => {
        loaded = true;
      });
  });
</script>

<svelte:head>
  <title>Dashboard</title>
</svelte:head>

<PageHeader
  label="Overview"
  title="Dashboard"
  description="Current status of your services and recent incidents."
>
  {#snippet actions()}
    <Button variant="outline" href="/maintenance" class="hidden sm:inline-flex">
      <Wrench />
      Schedule maintenance
    </Button>
    <Button href="/services/new">
      <Plus />
      New service
    </Button>
  {/snippet}
</PageHeader>

<div in:fly={{ x: 20, duration: 200 }} class="grid grid-cols-2 gap-3 sm:gap-4 lg:grid-cols-4">
  <StatCard label="Up" value={$stats.up} context="of {$stats.count} services" tone="up" />
  <StatCard label="Down" value={$stats.down} context="Failing checks" tone="down" />
  <StatCard label="Errors" value={$stats.failed} context="Check couldn't run" tone="pending" />
  <StatCard label="Paused" value={paused} context="Not being checked" tone="neutral" />
</div>

<div in:fly={{ y: 20, duration: 200 }}>
  <Card.Root class="gap-0 py-0">
    <div class="flex items-center gap-2.5 border-b px-4 py-4 sm:px-5">
      <h2 class="text-lg font-bold tracking-[-0.015em]">Recent incidents</h2>
      {#if $data.length}
        <span
          class="rounded-full bg-raised px-2 py-px text-xs font-semibold text-muted-foreground tabular-nums"
        >
          {$data.length}
        </span>
      {/if}
    </div>

    {#if loaded && $data.length === 0}
      <EmptyState
        icon={CircleCheck}
        message="No incidents. All services are running normally."
        framed={false}
      >
        <Button href="/services">View services</Button>
      </EmptyState>
    {:else}
      <div class="flex flex-col gap-2 border-b px-4 py-3 sm:flex-row sm:items-center sm:px-5">
        <div class="relative sm:max-w-xs sm:flex-1">
          <Search
            class="pointer-events-none absolute top-1/2 left-3 size-4 -translate-y-1/2 text-muted-foreground"
            aria-hidden="true"
          />
          <Input
            type="search"
            placeholder="Search incidents…"
            aria-label="Search incidents"
            class="h-9 pl-9"
            bind:value={$filterValue}
          />
        </div>
        <div class="flex *:flex-1 sm:ml-auto sm:*:flex-none">
          <DateFilter {days} bind:value={selectedDays} />
        </div>
      </div>

      <!-- Table from sm up -->
      <div class="hidden sm:block">
        <Table.Root {...$tableAttrs}>
          <Table.Header>
            {#each $headerRows as headerRow (headerRow.id)}
              <Subscribe rowAttrs={headerRow.attrs()} let:rowAttrs>
                <Table.Row {...rowAttrs} class="bg-surface hover:bg-surface">
                  {#each headerRow.cells as cell (cell.id)}
                    <Subscribe attrs={cell.attrs()} let:attrs>
                      <Table.Head {...attrs} class={alignEnd(cell.id)}>
                        <Render of={cell.render()} />
                      </Table.Head>
                    </Subscribe>
                  {/each}
                </Table.Row>
              </Subscribe>
            {/each}
          </Table.Header>
          <Table.Body {...$tableBodyAttrs}>
            {#each $pageRows as row (row.id)}
              <Subscribe rowAttrs={row.attrs()} let:rowAttrs>
                <Table.Row {...rowAttrs} class="last:border-b-0">
                  {#each row.cells as cell (cell.id)}
                    <Subscribe attrs={cell.attrs()} let:attrs>
                      <Table.Cell {...attrs} class="tabular-nums {cellClass(cell.id)}">
                        <Render of={cell.render()} />
                      </Table.Cell>
                    </Subscribe>
                  {/each}
                </Table.Row>
              </Subscribe>
            {:else}
              <Table.Row class="hover:bg-transparent">
                <Table.Cell colspan={columns.length} class="h-24 text-center text-muted-foreground">
                  {loaded ? 'No incidents match your filters.' : 'Loading incidents…'}
                </Table.Cell>
              </Table.Row>
            {/each}
          </Table.Body>
        </Table.Root>
      </div>

      <!-- Stacked list on mobile -->
      <ul class="divide-y sm:hidden">
        {#each $pageRows as row (row.id)}
          {#if row.isData()}
            {@const incident = row.original}
            <li class="flex flex-col gap-1 px-4 py-3">
              <div class="flex items-center justify-between gap-2">
                <ServiceCell id={incident.service_id} name={incident.service_name} />
                <StatusCell statusId={incident.status} />
              </div>
              <p class="line-clamp-2 text-sm text-muted-foreground">{incident.messages}</p>
              <p class="text-xs text-muted-foreground tabular-nums">
                {formatStarted(incident.start)} · {formatDuration(incident.start, incident.end)} ·
                {incident.count}
                {incident.count === 1 ? 'check' : 'checks'}
              </p>
            </li>
          {/if}
        {:else}
          <li class="px-4 py-8 text-center text-sm text-muted-foreground">
            {loaded ? 'No incidents match your filters.' : 'Loading incidents…'}
          </li>
        {/each}
      </ul>

      {#if $pageCount > 1}
        <div class="flex items-center justify-between gap-2 border-t px-4 py-3 sm:px-5">
          <p class="text-sm text-muted-foreground tabular-nums">
            Page {$pageIndex + 1} of {$pageCount} · {$rows.length} incidents
          </p>
          <div class="flex gap-2">
            <Button
              variant="outline"
              size="icon-sm"
              disabled={!$hasPreviousPage}
              onclick={() => pageIndex.update((i) => i - 1)}
            >
              <ChevronLeft />
              <span class="sr-only">Previous page</span>
            </Button>
            <Button
              variant="outline"
              size="icon-sm"
              disabled={!$hasNextPage}
              onclick={() => pageIndex.update((i) => i + 1)}
            >
              <ChevronRight />
              <span class="sr-only">Next page</span>
            </Button>
          </div>
        </div>
      {/if}
    {/if}
  </Card.Root>
</div>
