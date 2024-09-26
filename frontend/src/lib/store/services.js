import { writable } from 'svelte/store';

/**
 * @typedef {Object} Service
 * @property {number} id - The ID.
 * @property {boolean} active - The service active status.
 * @property {string} name - The service name.
 * @property {string} url - The service URL.
 * @property {string} service_type - The service type.
 * @property {number} last_status - The last status of this service.
 * @property {number} timeout - The timeout when checking service.
 * @property {number} retry - The number of retries.
 * @property {number} retry_interval - The retry interval in seconds.
 */

/** @type {import('svelte/store').Writable<Service[]>} */
export default writable([]);
