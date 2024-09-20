<script>
  import {
    VisArea,
    VisAxis,
    VisCrosshair,
    VisLine,
    VisScatter,
    VisTooltip,
    VisXYContainer
  } from '@unovis/svelte';
  import {
    crosshairPointColors,
    crosshairStrokeWidths,
    lineColors,
    scatterPointColors,
    scatterPointStrokeColors,
    tooltipTemplate
  } from './helpers.js';
  import { onMount } from 'svelte';
  import { writable } from 'svelte/store';
  import { toDate, formatRelative, format } from 'date-fns';

  /**
   * @typedef {Object} Log
   * @property {number} service_id - The id of the service.
   * @property {number} status - The status.
   * @property {string} time - Timestamp.
   * @property {number} duration - Time taken.
   */

  /** @type {import('svelte/store').Writable<Log[]>} */
  export let data = writable([]);

  /** @type {number} */
  export let interval = 60

  /**
   * Returns the ID of the data point.
   * @param {Log} d - The data point.
   * @returns {number} - The timestamp.
   */
  const x = (d) => {
    const time = toDate(d.time);
    return time.getTime() / interval;
  };

  /**
   * Returns the ID of the data point.
   * @param {number} tick - The data point.
   * @param {number} i - The data index.
   * @returns {string} - Formated time.
   */
  const tickFormat = (tick, i) => {
    const time = toDate(tick * interval);
    return `${formatRelative(time, new Date())}`;
  };

  /**
   * Returns an array of functions that return the today and average values of the data point.
   * @param {Log} d - The data point.
   * @returns {number[]} - The today and average values.
   */
  // @ts-ignore
  const y = [(d) => d.duration];

  /** @type {Element} */
  let boxEl;

  /**
   * Adds a new random data point to the data array every 5 seconds.
   */
  onMount(() => {
    const resizeObserver = new ResizeObserver((entries) => {
      // We're only watching one element
      const entry = entries.at(0);
    });

    resizeObserver.observe(boxEl);

    return () => {
      resizeObserver.unobserve(boxEl);
    };
  });
</script>

<svelte:head>
  <style>
    :root {
      --vis-color0: var(--primary);
      --vis-color1: var(--secondary);
      --vis-color2: #ffffff;
      --vis-color3: #ffffff;
      --vis-color4: #ffffff;
      --vis-area-fill-opacity: 0.8;
    }
    .vis-xy-container {
      --vis-tooltip-padding: '0px';
      --vis-tooltip-background-color: 'transparent';
      --vis-tooltip-border-color: 'transparent';
      --vis-axis-grid-color: hsl(var(--foreground) / 0.05);
      --vis-crosshair-circle-stroke-color: hsl(var(--primary) / 50);
    }
  </style>
</svelte:head>

<svg class="h-0">
  <defs>
    <linearGradient id="gradient" gradientTransform="rotate(90)">
      <stop offset="40%" stop-color="hsl(var(--primary))" />
      <stop offset="100%" stop-color="hsl(var(--primary) / 0.25)" />
    </linearGradient>
  </defs>
</svg>

<div bind:this={boxEl}>
  <VisXYContainer
    class="vis-xy-container"
    data={$data}
    height={200}
    margin={{ top: 5, right: 10, left: 10, bottom: 0 }}
  >
    <VisAxis domainLine={true} type="y" label="Latency (ms)" />
    <VisAxis type="x" label="Time" {tickFormat} />
    <VisArea {x} {y} color="url(#gradient)" />
    <VisTooltip />
    <VisLine {x} {y} lineWidth={2} color={lineColors} lineDashArray={[0]} />
    <VisScatter
      {x}
      {y}
      color="none"
      size={0}
      cursor="crosshair"
      strokeColor={scatterPointStrokeColors}
      strokeWidth={crosshairStrokeWidths}
    />
    <VisCrosshair {x} {y} template={tooltipTemplate} color={crosshairPointColors} />
  </VisXYContainer>
</div>