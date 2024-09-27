import { writable } from 'svelte/store';

/**
 * @typedef {Object} Notification
 * @property {boolean} beenRead - If the notification has been read.
 * @property {string} title - Notification title.
 * @property {string} message - Notification message.
 */

/** @type {import('svelte/store').Writable<Notification[]>} */
export default writable([]);
