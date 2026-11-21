/**
 * Status tones, see the status colours in docs/design/dashboard.md.
 * `neutral` is for paused or inactive things, `empty` for missing checks.
 *
 * @typedef {'up' | 'down' | 'pending' | 'maintenance' | 'neutral' | 'empty'} Tone
 */

// Full class names so Tailwind can find them.
/** @type {Record<Tone, string>} */
export const toneFill = {
  up: 'bg-status-up',
  down: 'bg-status-down',
  pending: 'bg-status-pending',
  maintenance: 'bg-status-maintenance',
  neutral: 'bg-muted-foreground',
  empty: 'bg-raised'
};

/** @type {Record<Tone, string>} */
export const toneTint = {
  up: 'bg-status-up-tint',
  down: 'bg-status-down-tint',
  pending: 'bg-status-pending-tint',
  maintenance: 'bg-status-maintenance-tint',
  neutral: 'bg-raised',
  empty: 'bg-raised'
};

/** @type {Record<Tone, string>} */
export const toneText = {
  up: 'text-status-up',
  down: 'text-status-down',
  pending: 'text-status-pending',
  maintenance: 'text-status-maintenance',
  neutral: 'text-muted-foreground',
  empty: 'text-muted-foreground'
};

/** @type {Record<Tone, string>} */
export const toneBorder = {
  up: 'border-status-up',
  down: 'border-status-down',
  pending: 'border-status-pending',
  maintenance: 'border-status-maintenance',
  neutral: 'border-muted-foreground',
  empty: 'border-border'
};
