<script>
  import { get, writable } from 'svelte/store';
  import { onMount } from 'svelte';
  import { fly } from 'svelte/transition';
  import { Render, Subscribe, createRender, createTable } from 'svelte-headless-table';
  import {
    addColumnFilters,
    addHiddenColumns,
    addPagination,
    addSelectedRows,
    addSortBy,
    addTableFilter,
    addResizedColumns
  } from 'svelte-headless-table/plugins';

  import stats from '$lib/store/stats.js';
  import { cfetch } from '$lib/utils.js';

  import * as Table from '$lib/components/ui/table/index.js';
  import Activity from 'lucide-svelte/icons/activity';
  import Timer from 'lucide-svelte/icons/timer';
  import CircleArrowUp from 'lucide-svelte/icons/circle-arrow-up';
  import CircleArrowDown from 'lucide-svelte/icons/circle-arrow-down';
  import * as Card from '$lib/components/ui/card/index.js';

  import MessageCell from './(components)/message-cell.svelte';
  import DurationCell from './(components)/duration-cell.svelte';
  import DateCell from './(components)/date-cell.svelte';
  import { goto } from '$app/navigation';
  import { toast } from 'svelte-sonner';

  /**
   * @typedef {Object} Incident
   * @property {number} service_id - The service id.
   * @property {string} service_name - The service name.
   * @property {string} service_url - The url for the service.
   * @property {number} status - The incident log status.
   * @property {string} date - The date of the incident.
   * @property {string} messages - The error messages.
   * @property {number} count - Number of checks.
   * @property {string} start - The start of the incident.
   * @property {string} end - The end of the incident.
   */

  /** @type {import('svelte/store').Writable<Incident[]>} */
  export const data = writable([]);

  const table = createTable(data, {
    select: addSelectedRows(),
    sort: addSortBy({
      toggleOrder: ['asc', 'desc']
    }),
    page: addPagination(),
    filter: addTableFilter({
      fn: ({ filterValue, value }) => {
        return value.toLowerCase().includes(filterValue.toLowerCase());
      }
    }),
    colFilter: addColumnFilters(),
    hide: addHiddenColumns(),
    resize: addResizedColumns()
  });

  const columns = table.createColumns([
    table.column({
      header: 'Service Name',
      accessor: 'service_name',
      id: 'name'
    }),
    table.column({
      header: 'Message',
      accessor: 'messages',
      id: 'message',
      cell: ({ value, row }) => {
        if (row.isData()) {
          return createRender(MessageCell, {
            message: value,
            statusId: row.original.status
          });
        }
        return value;
      }
    }),
    table.column({
      header: 'Duration',
      accessor: 'start',
      id: 'duration',
      cell: ({ row }) => {
        if (row.isData()) {
          return createRender(DurationCell, {
            start: row.original.start,
            end: row.original.end
          });
        }
        return '';
      },
      plugins: {
        sort: {
          disable: true
        }
      }
    }),
    table.column({
      header: 'Checks count',
      accessor: 'count',
      id: 'count'
    }),
    table.column({
      header: 'Date',
      accessor: 'date',
      id: 'date',
      cell: ({ value, row }) => {
        if (row.isData()) {
          return createRender(DateCell, {
            date: value
          });
        }
        return value;
      },
      plugins: {
        colFilter: {
          fn: ({ filterValue, value }) => {
            if (filterValue.length === 0) return true;
            if (!Array.isArray(filterValue) || typeof value !== 'string') return true;
            return filterValue.some((filter) => {
              return value.includes(filter);
            });
          },
          initialFilterValue: [],
          render: ({ filterValue }) => {
            return get(filterValue);
          }
        }
      }
    })
  ]);

  const tableModel = table.createViewModel(columns);

  const { headerRows, pageRows, tableAttrs, tableBodyAttrs } = tableModel;

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
      });
  });
</script>

<svelte:head>
  <title>Home</title>
  <meta name="description" content="Svelte demo app" />
</svelte:head>

<div in:fly={{ x: 20, duration: 200 }} class="grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
  <Card.Root class="border-green-500/50 bg-green-500/10 dark:bg-green-500/20">
    <Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
      <Card.Title class="text-sm font-medium">Up Services</Card.Title>
      <CircleArrowUp class="h-4 w-4 text-muted-foreground" />
    </Card.Header>
    <Card.Content>
      <div class="text-2xl font-bold">{$stats.up}</div>
      <p class="text-xs text-muted-foreground">+20.1% from last month</p>
    </Card.Content>
  </Card.Root>
  <Card.Root class="border-red-500/50 bg-red-500/10 dark:bg-red-500/20">
    <Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
      <Card.Title class="text-sm font-medium">Down</Card.Title>
      <CircleArrowDown class="h-4 w-4 text-muted-foreground" />
    </Card.Header>
    <Card.Content>
      <div class="text-2xl font-bold">{$stats.down}</div>
      <p class="text-xs text-muted-foreground">+180.1% from last month</p>
    </Card.Content>
  </Card.Root>
  <Card.Root class="border-yellow-500/50 bg-yellow-500/10 dark:bg-yellow-500/20">
    <Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
      <Card.Title class="text-sm font-medium">Pending</Card.Title>
      <Timer class="h-4 w-4 text-muted-foreground" />
    </Card.Header>
    <Card.Content>
      <div class="text-2xl font-bold">{$stats.failed}</div>
      <p class="text-xs text-muted-foreground">+19% from last month</p>
    </Card.Content>
  </Card.Root>
  <Card.Root class="border-primary/50 bg-primary/10 dark:bg-primary/20">
    <Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
      <Card.Title class="text-sm font-medium">Active Services</Card.Title>
      <Activity class="h-4 w-4 text-muted-foreground" />
    </Card.Header>
    <Card.Content>
      <div class="text-2xl font-bold">{$stats.active}</div>
      <p class="text-xs text-muted-foreground">+201 since last hour</p>
    </Card.Content>
  </Card.Root>
</div>

<div in:fly={{ y: 20, duration: 200 }} class="h-full space-y-4">
  <div class="flex flex-col justify-between space-y-2 md:flex-row md:items-center">
    <h2 class="text-2xl font-bold tracking-tight">Incidents</h2>
    <p class="text-muted-foreground">Here's a list of recent incidents!</p>
  </div>
  <div class="overflow-hidden rounded-md border bg-background dark:border-primary/50">
    <Table.Root {...$tableAttrs}>
      <Table.Header>
        {#each $headerRows as headerRow (headerRow.id)}
          <Subscribe rowAttrs={headerRow.attrs()} let:rowAttrs>
            <Table.Row {...rowAttrs}>
              {#each headerRow.cells as cell (cell.id)}
                <Subscribe attrs={cell.attrs()} let:attrs props={cell.props()}>
                  <Table.Head class="py-1" {...attrs}>
                    {#if cell.id !== 'select' && cell.id !== 'actions'}
                      <Render of={cell.render()} />
                    {:else}
                      <Render of={cell.render()} />
                    {/if}
                  </Table.Head>
                </Subscribe>
              {/each}
            </Table.Row>
          </Subscribe>
        {/each}
      </Table.Header>
      <Table.Body {...$tableBodyAttrs}>
        {#if $pageRows.length}
          {#each $pageRows as row (row.id)}
            <Subscribe rowAttrs={row.attrs()} let:rowAttrs>
              <Table.Row {...rowAttrs}>
                {#each row.cells as cell (cell.id)}
                  <Subscribe attrs={cell.attrs()} let:attrs>
                    <Table.Cell class="cursor-pointer py-1" {...attrs}>
                      <Render of={cell.render()} />
                    </Table.Cell>
                  </Subscribe>
                {/each}
              </Table.Row>
            </Subscribe>
          {/each}
        {:else}
          <Table.Row>
            <Table.Cell colspan={columns.length} class="h-24 text-center"
              >Must be your lucky day.<br />All Good No Incidents.</Table.Cell
            >
          </Table.Row>
        {/if}
      </Table.Body>
    </Table.Root>
  </div>
</div>
