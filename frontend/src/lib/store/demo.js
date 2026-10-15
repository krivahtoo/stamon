import { writable } from 'svelte/store';

/**
 * Whether this instance is the public demo, from `/api/check`.
 * @type {import('svelte/store').Writable<boolean>}
 */
export default writable(false);
