export const StatusValue = {
  Pending: 0,
  Up: 1,
  Down: 2,
  Failed: 3,
};

/**
 * @typedef {Object} Status
 * @property {number} value - The integer representation of the status.
 * @property {string} label - status label
 * @property {string} variant - variant to use for the badge
 */

/** @type {Status[]} */
export const statuses = [
  {
    value: StatusValue.Pending,
    label: 'Pending',
    variant: 'outline',
  },
  {
    value: StatusValue.Up,
    label: 'Up',
    variant: 'default',
  },
  {
    value: StatusValue.Down,
    label: 'Down',
    variant: 'destructive',
  },
  {
    value: StatusValue.Failed,
    label: 'Failed',
    variant: 'secondary',
  },
];