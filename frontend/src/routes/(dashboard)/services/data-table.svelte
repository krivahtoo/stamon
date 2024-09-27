<script>
  import { get } from 'svelte/store';
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
  import services from '$lib/store/services.js';
  import DataTableStatusCell from '$lib/components/table/status-cell.svelte';

  import * as Table from '$lib/components/ui/table/index.js';
  import DataTableStatusBar from './(components)/data-table-status-bar.svelte';
  import DataTableCheckbox from './(components)/data-table-checkbox.svelte';
  import DataTableRowActions from './(components)/data-table-row-actions.svelte';
  import DataTablePagination from './(components)/data-table-pagination.svelte';
  import DataTableToolbar from './(components)/data-table-toolbar.svelte';
  import DataTableColumnHeader from './(components)/data-table-column-header.svelte';
  import { fly } from 'svelte/transition';

  const table = createTable(services, {
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
      header: 'Recent Status',
      accessor: 'id',
      cell: ({ value, row }) => {
        if (row.isData()) {
          return createRender(DataTableStatusBar, {
            serviceId: value
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
      header: 'Status',
      accessor: 'last_status',
      cell: ({ value, row }) => {
        if (row.isData()) {
          return createRender(DataTableStatusCell, {
            statusId: value || 0
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
      accessor: 'service_type',
      id: 'service_type',
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
            service: row.original
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

<div in:fly={{ y: 20, duration: 200 }} class="h-full space-y-4">
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
                          goto(`/services/${row.original.id}`);
                        }
                      }}
                      class="cursor-pointer py-1"
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
            <Table.Cell colspan={columns.length} class="h-24 text-center">
              No services here.<br />
              Add a new service
            </Table.Cell>
          </Table.Row>
        {/if}
      </Table.Body>
    </Table.Root>
  </div>
  <DataTablePagination {tableModel} />
</div>
