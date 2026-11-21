<script>
  import { Area, Axis, Chart, Grid, Highlight, Layer, Spline, Tooltip } from 'layerchart';
  import * as Card from '$lib/components/ui/card/index.js';
  import * as ChartUI from '$lib/components/ui/chart/index.js';
  import { cfetch, cn, formatTime } from '$lib/utils.js';

  /**
   * @typedef {'1h' | '24h' | '7d' | '30d'} Range
   *
   * @typedef {Object} Bucket
   * @property {string} start
   * @property {number | null} avg - Mean latency of the up checks, in ms.
   * @property {number | null} min
   * @property {number | null} max
   * @property {number} up
   * @property {number} down
   *
   * @typedef {Object} Latency
   * @property {Range} range
   * @property {number} bucket_secs
   * @property {Bucket[]} buckets - Oldest first, without empty buckets.
   * @property {number | null} avg
   * @property {number | null} min
   * @property {number | null} max
   *
   * A bucket on the chart. `gap` points have no checks and only break the line.
   * @typedef {Object} Point
   * @property {Date} start
   * @property {Date} end
   * @property {number | null} avg
   * @property {number | null} min
   * @property {number | null} max
   * @property {number} down
   * @property {boolean} [gap]
   */

  /**
   * @typedef {Object} Props
   * @property {number} serviceId
   */

  /** @type {Props} */
  let { serviceId } = $props();

  /** @type {{ value: Range, hours: number }[]} */
  const ranges = [
    { value: '1h', hours: 1 },
    { value: '24h', hours: 24 },
    { value: '7d', hours: 24 * 7 },
    { value: '30d', hours: 24 * 30 }
  ];

  const config = {
    latency: { label: 'Latency', color: 'var(--primary)' },
    down: { label: 'Down', color: 'var(--status-down)' }
  };

  /** @type {Range} */
  let range = $state('24h');
  /** @type {Latency | null} */
  let latency = $state(null);
  /** @type {string | null} */
  let error = $state(null);
  /** When the shown response times were loaded. */
  let loadedAt = $state(Date.now());

  $effect(() => {
    const requested = range;
    error = null;
    cfetch(`/services/${serviceId}/latency?range=${requested}`)
      .then(async (res) => {
        if (range !== requested) return;
        if (res.ok) {
          latency = await res.json();
          loadedAt = Date.now();
        } else {
          error = `Could not load response times (${res.status})`;
        }
      })
      .catch((e) => {
        if (range === requested) error = `Could not load response times: ${e?.message ?? e}`;
      });
  });

  // From the loaded data, so the axes don't change before it does.
  let hours = $derived(ranges.find((r) => r.value === (latency?.range ?? range))?.hours ?? 24);
  // The domain is fixed so quiet stretches show as empty space.
  let domain = $derived([new Date(loadedAt - hours * 3600 * 1000), new Date(loadedAt)]);

  /**
   * Buckets as chart points, with a gap point wherever no checks ran for more
   * than two buckets, so the line breaks there instead of bridging it.
   */
  let points = $derived.by(() => {
    if (!latency) return [];
    const size = latency.bucket_secs * 1000;
    /** @type {Point[]} */
    const points = [];
    for (const bucket of latency.buckets) {
      const start = new Date(bucket.start);
      const previous = points.at(-1);
      if (previous && start.getTime() - previous.start.getTime() > 2 * size) {
        const middle = new Date(previous.end.getTime());
        points.push({
          start: middle,
          end: middle,
          avg: null,
          min: null,
          max: null,
          down: 0,
          gap: true
        });
      }
      points.push({ ...bucket, start, end: new Date(start.getTime() + size) });
    }
    return points;
  });

  /** Buckets with down checks, with neighbouring ones merged. */
  let downBands = $derived.by(() => {
    /** @type {{ start: Date, end: Date }[]} */
    const bands = [];
    for (const point of points) {
      if (!point.down) continue;
      const last = bands.at(-1);
      if (last && last.end.getTime() >= point.start.getTime()) {
        last.end = point.end;
      } else {
        bands.push({ start: point.start, end: point.end });
      }
    }
    return bands;
  });

  /** @param {Date} time */
  const tickFormat = (time) =>
    hours <= 24
      ? formatTime(time, { hour: '2-digit', minute: '2-digit', hourCycle: 'h23' })
      : formatTime(time, { month: 'short', day: 'numeric' });

  /** @param {Point} point */
  function bucketLabel(point) {
    const sameDay = hours <= 24;
    /** @type {Intl.DateTimeFormatOptions} */
    const time = { hour: '2-digit', minute: '2-digit', hourCycle: 'h23' };
    const start = formatTime(
      point.start,
      sameDay ? time : { month: 'short', day: 'numeric', ...time }
    );
    return `${start}–${formatTime(point.end, time)}`;
  }

  /** @param {number | null | undefined} ms */
  const formatMs = (ms) => (ms == null ? '–' : `${Math.round(ms)} ms`);
</script>

<Card.Root class="gap-0 py-0">
  <div class="flex items-center justify-between gap-3 border-b px-4 py-3 sm:px-5">
    <h2 class="font-semibold">Response time</h2>
    <div role="radiogroup" aria-label="Range" class="flex rounded-md bg-raised p-0.5">
      {#each ranges as option (option.value)}
        <button
          type="button"
          role="radio"
          aria-checked={range === option.value}
          class={cn(
            'rounded-sm px-2.5 py-1 text-xs font-medium text-muted-foreground transition-colors',
            range === option.value && 'bg-background font-semibold text-foreground shadow-xs'
          )}
          onclick={() => (range = option.value)}
        >
          {option.value}
        </button>
      {/each}
    </div>
  </div>

  <div class="px-4 pt-4 pb-3 sm:px-5">
    {#if error}
      <p class="py-16 text-center text-sm text-destructive">{error}</p>
    {:else if latency && !latency.buckets.length}
      <p class="py-16 text-center text-sm text-muted-foreground">No checks in this range yet.</p>
    {:else}
      <ChartUI.Container {config} class="aspect-auto h-48 w-full sm:h-56">
        <Chart
          data={points}
          x="start"
          y="avg"
          xDomain={domain}
          yDomain={[0, null]}
          yNice
          padding={{ left: 32, bottom: 20, top: 4 }}
          tooltipContext={{ mode: 'bisect-x' }}
        >
          {#snippet children({ context })}
            <Layer type="svg">
              {#each downBands as band (band.start.getTime())}
                {@const x = context.xScale(band.start)}
                <rect
                  {x}
                  y={0}
                  width={Math.max(2, context.xScale(band.end) - x)}
                  height={context.height}
                  class="fill-(--color-down) opacity-15"
                />
                <line
                  x1={x}
                  x2={x}
                  y1={0}
                  y2={context.height}
                  class="stroke-(--color-down)"
                  stroke-width="1.5"
                />
              {/each}
              <!-- Grid, not the axis grid: Chart.Container hides other rule lines. -->
              <Grid y yTicks={4} classes={{ line: 'stroke-border' }} />
              <Axis
                placement="left"
                ticks={4}
                format={(value) => `${value}`}
                classes={{ tickLabel: 'fill-muted-foreground font-mono' }}
              />
              <Axis
                placement="bottom"
                ticks={hours <= 24 ? 4 : 5}
                format={tickFormat}
                classes={{ tickLabel: 'fill-muted-foreground font-mono' }}
              />
              <Area
                defined={(/** @type {Point} */ d) => d.avg != null}
                class="fill-(--color-latency)"
                fillOpacity={0.12}
                line={false}
              />
              <Spline
                defined={(/** @type {Point} */ d) => d.avg != null}
                class="stroke-(--color-latency)"
                strokeWidth={1.5}
              />
              <Highlight points={{ class: 'fill-(--color-latency)' }} />
            </Layer>
            <Tooltip.Root variant="none">
              {#snippet children({ data })}
                {#if data && !data.gap}
                  <div
                    class="grid min-w-36 gap-1 rounded-lg border bg-background px-2.5 py-1.5 text-xs shadow-xl"
                  >
                    <p class="font-medium">{bucketLabel(data)}</p>
                    <p class="flex justify-between gap-4">
                      <span class="text-muted-foreground">Avg</span>
                      <span class="font-mono">{formatMs(data.avg)}</span>
                    </p>
                    {#if data.min != null && data.min !== data.max}
                      <p class="flex justify-between gap-4">
                        <span class="text-muted-foreground">Range</span>
                        <span class="font-mono">{data.min}–{data.max} ms</span>
                      </p>
                    {/if}
                    {#if data.down}
                      <p class="font-medium text-status-down">{data.down} down</p>
                    {/if}
                  </div>
                {/if}
              {/snippet}
            </Tooltip.Root>
          {/snippet}
        </Chart>
      </ChartUI.Container>
    {/if}
  </div>

  <div
    class="flex flex-wrap items-center justify-between gap-x-4 gap-y-1 px-4 pb-4 text-xs text-muted-foreground sm:px-5"
  >
    <p class="flex gap-4 font-mono">
      <span>Avg {formatMs(latency?.avg)}</span>
      <span>Min {formatMs(latency?.min)}</span>
      <span>Max {formatMs(latency?.max)}</span>
    </p>
    <p class="flex items-center gap-1.5">
      <span class="size-2.5 rounded-[2px] bg-status-down-tint" aria-hidden="true"></span>
      Down
    </p>
  </div>
</Card.Root>
