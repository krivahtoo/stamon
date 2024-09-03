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

  /**
   * @typedef {Object} Data
   * @property {number} average - The average value.
   * @property {number} today - The value for today.
   * @property {number} id - The ID.
   */

  /** @type {import('svelte/store').Writable<Data[]>} */
  let data = writable([]);

  /**
   * Returns the ID of the data point.
   * @param {Data} d - The data point.
   * @returns {number} - The ID.
   */
  const x = (d) => d.id;

  /**
   * Returns an array of functions that return the today and average values of the data point.
   * @param {Data} d - The data point.
   * @returns {number[]} - The today and average values.
   */
  // @ts-ignore
  const y = [(d) => d.average];

  /** @type {Element} */
  let boxEl;

  /**
   * Adds a new random data point to the data array every 5 seconds.
   */
  onMount(() => {
    const interval = setInterval(() => {
      data.update((d) => {
        const lastId = d.length ? d[d.length - 1].id : 0;
        const newData = {
          id: lastId + 1,
          average: Math.floor(Math.random() * 1000),
          today: Math.floor(Math.random() * 1000)
        };

        d.push(newData);

        if (d.length > 60) {
          d.shift();
        }
        return d;
      });
    }, 2000);
    const resizeObserver = new ResizeObserver((entries) => {
      // We're only watching one element
      const entry = entries.at(0);
    });

    resizeObserver.observe(boxEl);

    return () => {
      clearInterval(interval);
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
    height={100}
    margin={{ top: 5, right: 10, left: 10, bottom: 0 }}
  >
    <VisAxis domainLine={true} type="y" label="Time (ms)" />
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
    <VisCrosshair template={tooltipTemplate} color={crosshairPointColors} />
  </VisXYContainer>
</div>
