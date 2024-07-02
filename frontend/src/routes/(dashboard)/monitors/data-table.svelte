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

  import { goto } from '$app/navigation';

  import * as Table from '$lib/components/ui/table/index.js';
  import DataTableStatusBar from './(components)/data-table-status-bar.svelte';
  import DataTableCheckbox from './(components)/data-table-checkbox.svelte';
  import DataTableRowActions from './(components)/data-table-row-actions.svelte';
  import DataTablePagination from './(components)/data-table-pagination.svelte';
  import DataTableToolbar from './(components)/data-table-toolbar.svelte';
  import DataTableColumnHeader from './(components)/data-table-column-header.svelte';

  /**
   * @typedef {Object} Monitor
   * @property {number} id - The ID.
   * @property {boolean} active - The monitor active status.
   * @property {string} name - The monitor name.
   * @property {string} url - The monitor URL.
   * @property {string} monitor_type - The monitor type.
   * @property {number} retry - The number of retries.
   * @property {number} retry_interval - The retry interval in seconds.
   */

  /** @type {import('svelte-headless-table').ReadOrWritable<Monitor[]>} */
  export const data = readable([
    {
      id: 1,
      active: true,
      name: 'Google DNS',
      url: '8.8.8.8',
      monitor_type: 'ping',
      retry: 3,
      retry_interval: 60
    },
    {
      id: 2,
      active: true,
      name: 'Cloudflare DNS',
      url: '1.1.1.1',
      monitor_type: 'ping',
      retry: 3,
      retry_interval: 60
    },
    {
      id: 3,
      active: true,
      name: 'Cloudflare DNS',
      url: '1.1.1.1',
      monitor_type: 'ping',
      retry: 3,
      retry_interval: 60
    },
    {
      id: 4,
      active: true,
      name: 'Cloudflare DNS',
      url: '1.1.1.1',
      monitor_type: 'ping',
      retry: 3,
      retry_interval: 60
    },
    {
      id: 5,
      active: true,
      name: 'Cloudflare DNS',
      url: '1.1.1.1',
      monitor_type: 'ping',
      retry: 3,
      retry_interval: 60
    },
    {
      id: 6,
      active: true,
      name: 'Cloudflare DNS',
      url: '1.1.1.1',
      monitor_type: 'ping',
      retry: 3,
      retry_interval: 60
    },
    {
      id: 7,
      active: true,
      name: 'Cloudflare DNS',
      url: '1.1.1.1',
      monitor_type: 'ping',
      retry: 3,
      retry_interval: 60
    },
    {
      id: 8,
      active: true,
      name: 'Cloudflare DNS',
      url: '1.1.1.1',
      monitor_type: 'ping',
      retry: 3,
      retry_interval: 60
    },
    {
      id: 9,
      active: true,
      name: 'Google DNS',
      url: '8.8.8.8',
      monitor_type: 'ping',
      retry: 3,
      retry_interval: 60
    },
    {
      id: 10,
      active: true,
      name: 'Google DNS',
      url: '8.8.8.8',
      monitor_type: 'ping',
      retry: 3,
      retry_interval: 60
    },
    {
      id: 11,
      active: true,
      name: 'Google DNS',
      url: '8.8.8.8',
      monitor_type: 'ping',
      retry: 3,
      retry_interval: 60
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
    table.display({
      id: 'select',
      header: (_, { pluginStates }) => {
        const { allPageRowsSelected } = pluginStates.select;
        return createRender(DataTableCheckbox, {
          checked: allPageRowsSelected,
          'aria-label': 'Select all'
        });
      },
      cell: ({ row }, { pluginStates }) => {
        const { getRowState } = pluginStates.select;
        const { isSelected } = getRowState(row);
        return createRender(DataTableCheckbox, {
          checked: isSelected,
          'aria-label': 'Select row',
          class: 'translate-y-[2px]'
        });
      },
      plugins: {
        sort: {
          disable: true
        }
      }
    }),
    table.column({
      header: 'Name',
      accessor: 'name',
      id: 'name'
    }),
    table.column({
      header: 'Status',
      accessor: 'id',
      cell: ({ value, row }) => {
        if (row.isData()) {
          return createRender(DataTableStatusBar, {
            monitorId: value
          });
        }
        return value;
      },
      plugins: {
        sort: {
          disable: true
        }
      }
    }),
    table.column({
      header: 'Type',
      accessor: 'monitor_type',
      id: 'monitor_type',
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
    }),
    table.display({
      id: 'actions',
      header: () => {
        return '';
      },
      cell: ({ row }) => {
        if (row.isData() && row.original) {
          return createRender(DataTableRowActions, {
            monitor: row.original
          });
        }
        return '';
      },
      plugins: {
        sort: {
          disable: true
        }
      }
    })
  ]);

  const tableModel = table.createViewModel(columns);

  const { headerRows, pageRows, tableAttrs, tableBodyAttrs } = tableModel;
</script>

<div class="h-full space-y-4">
  <DataTableToolbar />
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
                      <DataTableColumnHeader {props} {tableModel} cellId={cell.id}>
                        <Render of={cell.render()} /></DataTableColumnHeader
                      >
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
                    <Table.Cell
                      on:click={() => {
                        if (cell.id !== 'select' && cell.id !== 'actions') {
                          goto(`/monitors/${row.original.id}`);
                        }
                      }}
                      class="py-1 cursor-pointer"
                      {...attrs}
                    >
                      <Render of={cell.render()} />
                    </Table.Cell>
                  </Subscribe>
                {/each}
              </Table.Row>
            </Subscribe>
          {/each}
        {:else}
          <Table.Row>
            <Table.Cell colspan={columns.length} class="h-24 text-center">No results.</Table.Cell>
          </Table.Row>
        {/if}
      </Table.Body>
    </Table.Root>
  </div>
  <DataTablePagination {tableModel} />
</div>
