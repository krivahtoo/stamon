<script>
  import * as Card from '$lib/components/ui/card/index.js';
  import * as Table from '$lib/components/ui/table/index.js';
  import { Button } from '$lib/components/ui/button/index.js';
  import { StatusPill } from '$lib/components/dashboard/index.js';
  import { statuses, StatusValue } from '$lib/data/table.js';
  import { cfetch, formatTime } from '$lib/utils.js';

  const PAGE_SIZE = 10;

  /**
   * @typedef {Object} Log
   * @property {number} id
   * @property {number} status
   * @property {string | null} message
   * @property {string} time
   * @property {number} duration - Time taken, in milliseconds.
   */

  /**
   * @typedef {Object} Props
   * @property {number} serviceId
   */

  /** @type {Props} */
  let { serviceId } = $props();

  /**
   * The `before` cursor of each page shown so far; the first page has none.
   * @type {(number | undefined)[]}
   */
  let cursors = $state([undefined]);
  /** @type {Log[]} */
  let logs = $state([]);
  let loading = $state(true);
  /** @type {string | null} */
  let error = $state(null);

  let page = $derived(cursors.length - 1);

  $effect(() => {
    const before = cursors[page];
    const query = `limit=${PAGE_SIZE}${before === undefined ? '' : `&before=${before}`}`;
    loading = true;
    cfetch(`/services/${serviceId}/logs?${query}`)
      .then(async (res) => {
        if (cursors[page] !== before) return;
        if (res.ok) {
          logs = (await res.json()).logs;
          error = null;
        } else {
          error = `Could not load check history (${res.status})`;
        }
      })
      .catch((e) => (error = `Could not load check history: ${e?.message ?? e}`))
      .finally(() => (loading = false));
  });

  /** @param {Log} log */
  const statusOf = (log) => statuses.find((s) => s.value === log.status);

  /** @param {Log} log */
  const latencyOf = (log) => (log.status === StatusValue.Up ? `${log.duration} ms` : '–');

  /**
   * The time of day, with the date for checks before today.
   * @param {string} time
   */
  function checkedAt(time) {
    /** @type {Intl.DateTimeFormatOptions} */
    const clock = { hour: '2-digit', minute: '2-digit', second: '2-digit', hourCycle: 'h23' };
    /** @type {Intl.DateTimeFormatOptions} */
    const day = { year: 'numeric', month: '2-digit', day: '2-digit' };
    return formatTime(time, day) === formatTime(new Date(), day)
      ? formatTime(time, clock)
      : formatTime(time, { month: 'short', day: 'numeric', ...clock });
  }

  let first = $derived(page * PAGE_SIZE + 1);
</script>

<Card.Root class="gap-0 py-0">
  <div class="flex items-baseline gap-3 border-b px-4 py-3 sm:px-5">
    <h2 class="font-semibold">Check history</h2>
    <p class="text-sm text-muted-foreground">Most recent first</p>
  </div>

  {#if error}
    <p class="px-5 py-8 text-center text-sm text-destructive">{error}</p>
  {:else if !loading && logs.length === 0}
    <p class="px-5 py-8 text-center text-sm text-muted-foreground">No checks yet.</p>
  {:else}
    <!-- Table from sm up -->
    <div class="hidden sm:block">
      <Table.Root>
        <Table.Header>
          <Table.Row class="bg-surface hover:bg-surface">
            <Table.Head>Status</Table.Head>
            <Table.Head class="w-full">Message</Table.Head>
            <Table.Head class="text-right">Latency</Table.Head>
            <Table.Head class="text-right">Checked</Table.Head>
          </Table.Row>
        </Table.Header>
        <Table.Body>
          {#each logs as log (log.id)}
            {@const status = statusOf(log)}
            <Table.Row class="last:border-b-0">
              <Table.Cell class="h-12">
                <StatusPill tone={status?.tone}>{status?.label ?? 'Unknown'}</StatusPill>
              </Table.Cell>
              <Table.Cell class="max-w-0 truncate">{log.message ?? '–'}</Table.Cell>
              <Table.Cell class="text-right font-mono text-muted-foreground tabular-nums">
                {latencyOf(log)}
              </Table.Cell>
              <Table.Cell class="text-right font-mono text-muted-foreground tabular-nums">
                {checkedAt(log.time)}
              </Table.Cell>
            </Table.Row>
          {/each}
        </Table.Body>
      </Table.Root>
    </div>

    <!-- Stacked rows on mobile -->
    <ul class="divide-y sm:hidden">
      {#each logs as log (log.id)}
        {@const status = statusOf(log)}
        <li class="flex flex-col gap-1.5 px-4 py-3">
          <div class="flex items-center justify-between gap-2">
            <StatusPill tone={status?.tone}>{status?.label ?? 'Unknown'}</StatusPill>
            <span class="font-mono text-xs text-muted-foreground tabular-nums">
              {checkedAt(log.time)}
            </span>
          </div>
          {#if log.message || log.status === StatusValue.Up}
            <p class="flex justify-between gap-2 text-sm">
              <span class="min-w-0 truncate">{log.message ?? ''}</span>
              {#if log.status === StatusValue.Up}
                <span class="shrink-0 font-mono text-xs text-muted-foreground"
                  >{latencyOf(log)}</span
                >
              {/if}
            </p>
          {/if}
        </li>
      {/each}
    </ul>
  {/if}

  <div class="flex items-center justify-between gap-2 border-t px-4 py-3 sm:px-5">
    <p class="text-sm text-muted-foreground tabular-nums">
      {#if logs.length}
        <span class="hidden sm:inline">Showing</span>
        {first}–{first + logs.length - 1}
      {/if}
    </p>
    <div class="flex gap-2">
      <Button
        variant="outline"
        size="sm"
        disabled={loading || page === 0}
        onclick={() => (cursors = cursors.slice(0, -1))}
      >
        Previous
      </Button>
      <Button
        variant="outline"
        size="sm"
        disabled={loading || logs.length < PAGE_SIZE}
        onclick={() => (cursors = [...cursors, logs.at(-1)?.id])}
      >
        Next
      </Button>
    </div>
  </div>
</Card.Root>
