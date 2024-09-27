/**
 * @typedef {Object} Status
 * @property {number} value - The integer representation of the status.
 * @property {string} label - status label
 * @property {string} variant - variant to use for the badge
 */

/** @type {Status[]} */
export const statuses = [
  {
    value: 0,
    label: 'Pending',
    variant: 'outline'
  },
  {
    value: 1,
    label: 'Up',
    variant: 'default'
  },
  {
    value: 2,
    label: 'Down',
    variant: 'destructive'
  },
  {
    value: 3,
    label: 'Failed',
    variant: 'secondary'
  }
];
