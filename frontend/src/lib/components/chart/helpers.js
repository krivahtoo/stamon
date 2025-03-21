import { toDate } from 'date-fns';
import { rtf } from '@sveu/shared';
import { get } from 'svelte/store';

/**
 * Returns a function that generates a color string with the specified opacity.
 * @param {string} [opacity="1"] - The opacity of the color.
 * @returns {function(): string} - A function that returns a color string.
 */
export function color(opacity = '1') {
  return () => `hsl(var(--primary) / ${opacity})`;
}

/**
 * @typedef {Object} Log
 * @property {number} service_id - The id of the service.
 * @property {number} status - The status.
 * @property {Date} time - Timestamp.
 * @property {number} duration - Time taken.
 */

/**
 * Returns a color string for a line based on its index.
 * @template T
 * @param {T} _ - An unused parameter.
 * @param {number} i - The index of the line.
 * @returns {string} - The color string.
 */
export function lineColors(_, i) {
  return ['hsl(var(--primary))', 'hsl(var(--primary) / 0.25)'][i];
}

/**
 * Returns a color string for a scatter point based on its index.
 * @template T
 * @param {T} _ - An unused parameter.
 * @param {number} i - The index of the scatter point.
 * @returns {string} - The color string.
 */
export function scatterPointColors(_, i) {
  return ['hsl(0, 0%, 100%)', 'hsl(var(--primary) / 0.25)'][i];
}

/**
 * Returns a stroke color string for a scatter point based on its index.
 * @template T
 * @param {T} _ - An unused parameter.
 * @param {number} i - The index of the scatter point.
 * @returns {string} - The stroke color string.
 */
export function scatterPointStrokeColors(_, i) {
  return ['hsl(var(--primary))', 'hsl(var(--primary) / 0.25)'][i];
}

/**
 * Returns a color string for a crosshair point based on its index.
 * @template T
 * @param {T} _ - An unused parameter.
 * @param {number} i - The index of the crosshair point.
 * @returns {string} - The color string.
 */
export function crosshairPointColors(_, i) {
  return ['hsl(var(--primary))', 'hsl(var(--primary) / 0.25)'][i];
}

/**
 * Returns a stroke width for a crosshair based on its index.
 * @template T
 * @param {T} _ - An unused parameter.
 * @param {number} i - The index of the crosshair.
 * @returns {number} - The stroke width.
 */
export function crosshairStrokeWidths(_, i) {
  return [2, 1][i];
}

/**
 * Generates an HTML template for a tooltip.
 * @param {Log} d - The log entry which has data to display in the tooltip.
 * @returns {string} - The HTML string for the tooltip.
 */
export function tooltipTemplate(d) {
  const time = rtf(d.time instanceof Date ? d.time : toDate(d.time));
  return `
<div class="rounded-lg border dark:border-primary/50 bg-background p-2 shadow-sm">
  <div class="grid grid-cols-1 gap-2">
    <div class="flex flex-col">
      <span class="text-[0.70rem] uppercase text-muted-foreground">
        Latency
      </span>
      <span class="font-bold text-muted-foreground">
        ${d.duration} ms
      </span>
    </div>
    <div class="flex flex-col">
      <span class="text-[0.70rem] uppercase text-muted-foreground">
        Time
      </span>
      <span class="font-bold text-foreground">
        ${get(time)}
      </span>
    </div>
  </div>
</div>
`;
}
