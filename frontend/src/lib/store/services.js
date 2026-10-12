import { writable } from 'svelte/store';

/**
 * @typedef {Object} Service
 * @property {number} id - The ID.
 * @property {boolean} active - The service active status.
 * @property {string} name - The service name.
 * @property {string} service_type - The check type, e.g. `http` or `ping`.
 * @property {string | null} target - The URL or host being checked; null for push monitors.
 * @property {Object} config - The type-specific check settings.
 * @property {string[]} tags - Labels for grouping and finding the service.
 * @property {number} last_status - The last status of this service.
 * @property {number} timeout - The timeout when checking service.
 * @property {number} retry - The number of retries.
 * @property {number} retry_interval - The retry interval in seconds.
 */

/** @type {import('svelte/store').Writable<Service[]>} */
export default writable([]);
