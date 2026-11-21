import { writable } from 'svelte/store';

/**
 * @typedef {Object} Instance
 * @property {string} version - The running Stamon version, from `/api/settings`.
 */

/** @type {import('svelte/store').Writable<null | Instance>} */
export default writable(null);
