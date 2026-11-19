import { differenceInSeconds, format, isToday, isYesterday } from 'date-fns';

/**
 * When an incident started: "Today, 14:02", "Yesterday, 22:41" or "Oct 14, 09:12".
 * @param {string} start
 */
export function formatStarted(start) {
  if (isToday(start)) return `Today, ${format(start, 'HH:mm')}`;
  if (isYesterday(start)) return `Yesterday, ${format(start, 'HH:mm')}`;
  return format(start, 'MMM d, HH:mm');
}

/**
 * How long an incident lasted, from its first to its last failed check:
 * "45s", "1m 30s", "2h 5m" or "3d 4h".
 * @param {string} start
 * @param {string} end
 */
export function formatDuration(start, end) {
  const seconds = Math.max(differenceInSeconds(end, start), 0);
  const d = Math.floor(seconds / 86400);
  const h = Math.floor((seconds % 86400) / 3600);
  const m = Math.floor((seconds % 3600) / 60);
  const s = seconds % 60;

  /** @param {[number, string][]} parts */
  const join = (parts) =>
    parts
      .filter(([value], i) => i === 0 || value > 0)
      .map(([value, unit]) => `${value}${unit}`)
      .join(' ');

  if (d) return join([[d, 'd'], [h, 'h']]);
  if (h) return join([[h, 'h'], [m, 'm']]);
  if (m) return join([[m, 'm'], [s, 's']]);
  return `${s}s`;
}

/**
 * A day option in the date filter, from an incident's `YYYY-MM-DD` date.
 * @param {string} date
 */
export function formatDay(date) {
  if (isToday(date)) return 'Today';
  if (isYesterday(date)) return 'Yesterday';
  return format(date, 'EEE, MMM d');
}
