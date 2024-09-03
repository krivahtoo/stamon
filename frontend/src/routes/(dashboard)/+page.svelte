<script>
  import { get, readable } from 'svelte/store';
  import { Render, Subscribe, createRender, createTable } from 'svelte-headless-table';
  import {
    addColumnFilters,
    addHiddenColumns,
    addPagination,
    addSelectedRows,
    addSortBy,
    addTableFilter
  } from 'svelte-headless-table/plugins';
  import { parseISO } from 'date-fns';

  import { goto } from '$app/navigation';

  import * as Table from '$lib/components/ui/table/index.js';
  import Activity from 'lucide-svelte/icons/activity';
  import CreditCard from 'lucide-svelte/icons/credit-card';
  import Timer from 'lucide-svelte/icons/timer';
  import Users from 'lucide-svelte/icons/users';
  import * as Card from '$lib/components/ui/card/index.js';
  import * as Select from '$lib/components/ui/select/index.js';

  /**
   * @typedef {Object} Monitor
   * @property {number} id - The ID.
   * @property {string} name - The monitor name.
   * @property {string} status - The monitor URL.
   * @property {string} message - The number of retries.
   * @property {string} time - The monitor type.
   */

  /** @type {import('svelte-headless-table').ReadOrWritable<Monitor[]>} */
  export const data = readable([
    {
      id: 1,
      name: 'Google DNS',
      status: 'Down',
      message: 'Connection failed',
      time: '2024-07-19 20:09:31'
    },
    {
      id: 2,
      name: 'Cloudflare',
      status: 'Down',
      message: 'Server returned 500',
      time: '2024-07-19 20:09:31'
    }
  ]);

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
    hide: addHiddenColumns()
  });

  const columns = table.createColumns([
    table.column({
      header: 'Service Name',
      accessor: 'name',
      id: 'name'
    }),
    table.column({
      header: 'Message',
      accessor: 'message',
      id: 'message'
    }),
    table.column({
      header: 'Status',
      accessor: 'status',
      plugins: {
        sort: {
          disable: true
        }
      }
    }),
    table.column({
      header: 'Time',
      accessor: 'time',
      id: 'time',
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
</script>

<svelte:head>
  <title>Home</title>
  <meta name="description" content="Svelte demo app" />
</svelte:head>

<div class="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
  <Card.Root class="border-green-500/50 bg-green-500/10 dark:bg-green-500/20">
    <Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
      <Card.Title class="text-sm font-medium">Up Services</Card.Title>
      <Timer class="h-4 w-4 text-muted-foreground" />
    </Card.Header>
    <Card.Content>
      <div class="text-2xl font-bold">12</div>
      <p class="text-xs text-muted-foreground">+20.1% from last month</p>
    </Card.Content>
  </Card.Root>
  <Card.Root class="border-red-500/50 bg-red-500/10 dark:bg-red-500/20">
    <Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
      <Card.Title class="text-sm font-medium">Down</Card.Title>
      <Users class="h-4 w-4 text-muted-foreground" />
    </Card.Header>
    <Card.Content>
      <div class="text-2xl font-bold">3</div>
      <p class="text-xs text-muted-foreground">+180.1% from last month</p>
    </Card.Content>
  </Card.Root>
  <Card.Root class="border-yellow-500/50 bg-yellow-500/10 dark:bg-yellow-500/20">
    <Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
      <Card.Title class="text-sm font-medium">Pending</Card.Title>
      <CreditCard class="h-4 w-4 text-muted-foreground" />
    </Card.Header>
    <Card.Content>
      <div class="text-2xl font-bold">1</div>
      <p class="text-xs text-muted-foreground">+19% from last month</p>
    </Card.Content>
  </Card.Root>
  <Card.Root class="border-primary/50 bg-primary/10 dark:bg-primary/20">
    <Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
      <Card.Title class="text-sm font-medium">Paused Services</Card.Title>
      <Activity class="h-4 w-4 text-muted-foreground" />
    </Card.Header>
    <Card.Content>
      <div class="text-2xl font-bold">3</div>
      <p class="text-xs text-muted-foreground">+201 since last hour</p>
    </Card.Content>
  </Card.Root>
</div>

<div class="h-full space-y-4">
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
                <Subscribe attrs={cell.attrs()} let:attrs props={cell.props()} let:props>
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