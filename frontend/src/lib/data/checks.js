import { toDate, formatRFC7231 } from 'date-fns';
import { cfetch } from '$lib/utils.js';
import { statuses, StatusValue } from './table.js';

/** Display names of the check types. */
/** @type {Record<string, string>} */
export const typeLabels = {
  http: 'HTTP',
  ping: 'Ping',
  tcp: 'TCP',
  dns: 'DNS',
  tls: 'TLS',
  docker: 'Docker',
  database: 'Database',
  push: 'Push'
};

/** @param {string} type */
export const typeLabel = (type) => typeLabels[type] ?? type;

/**
 * @typedef {Object} Log
 * @property {number} service_id - The id of the service.
 * @property {number} status - The status.
 * @property {string} time - Timestamp.
 * @property {number} duration - Time taken, in milliseconds.
 */

/**
 * @typedef {Object} RecentChecks
 * @property {{ tone: import('$lib/components/dashboard/tones.js').Tone, label?: string }[]} ticks - Oldest first, for UptimeBar.
 * @property {number} [uptime] - Percentage of these checks that were up.
 * @property {number} [latency] - Duration of the last check, when it was up.
 * @property {string} [lastCheck] - Time of the last check.
 */

/**
 * The most recent checks of a service, as shown in its uptime bar.
 * @param {number} serviceId
 * @param {number} [limit]
 * @returns {Promise<RecentChecks>}
 */
export async function loadRecentChecks(serviceId, limit = 32) {
  const res = await cfetch(`/services/${serviceId}/logs?limit=${limit}`);
  if (!res.ok) return { ticks: [] };

  /** @type {Log[]} */
  const logs = (await res.json()).logs.reverse();
  const last = logs.at(-1);

  return {
    ticks: logs.map((log) => ({
      tone: statuses.find((status) => status.value === log.status)?.tone ?? 'empty',
      label: formatRFC7231(toDate(log.time))
    })),
    uptime: logs.length
      ? (logs.filter((log) => log.status === StatusValue.Up).length / logs.length) * 100
      : undefined,
    latency: last?.status === StatusValue.Up ? last.duration : undefined,
    lastCheck: last?.time
  };
}
