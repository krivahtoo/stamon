import { writable } from 'svelte/store';

/**
 * @typedef {Object} User
 * @property {number} id - The ID.
 * @property {string} username - user name
 * @property {string} role - User's role in the system.
 * @property {boolean} active - If this account is active.
 */

/** @type {import('svelte/store').Writable<null | User>} */
export default writable(null);