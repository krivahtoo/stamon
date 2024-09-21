import { writable } from 'svelte/store';

/**
 * @typedef {Object} Stats
 * @property {number} count - Total number of services.
 * @property {number} failed - Number of services that failed.
 * @property {number} up - Services which are up.
 * @property {number} down - Services which are down.
 * @property {number} active - Services that are actively being monitored.
 */

/** @type {import('svelte/store').Writable<Stats>} */
export default writable({
  count: 0,
  failed: 0,
  up: 0,
  down: 0,
  active: 0,
});