import { navGroups } from '$lib/data/nav.js';

/**
 * @typedef {Object} Crumb
 * @property {string} label - Text to show
 * @property {string} href - Where the crumb links to
 */

/**
 * Labels for path segments that aren't top-level nav items.
 * @type {Record<string, string>}
 */
const SEGMENT_LABELS = {
  new: 'New service',
  appearance: 'Appearance',
  general: 'General'
};

/** @type {Record<string, string>} */
const NAV_LABELS = Object.fromEntries(
  navGroups.flatMap((group) => group.items).map((item) => [item.path, item.name])
);

/**
 * The breadcrumb trail for a dashboard route, starting at Stamon.
 * The last crumb is the current page.
 * @param {string} pathname
 * @param {{ id: number, name: string }[]} [services] - Used to name `/services/<id>/`
 * @returns {Crumb[]}
 */
export function crumbs(pathname, services = []) {
  const segments = pathname.split('/').filter(Boolean);
  /** @type {Crumb[]} */
  const trail = [{ label: 'Stamon', href: '/' }];

  if (segments.length === 0) {
    trail.push({ label: NAV_LABELS['/'], href: '/' });
    return trail;
  }

  let href = '/';
  segments.forEach((segment, idx) => {
    href += `${segment}/`;
    let label = NAV_LABELS[href] ?? SEGMENT_LABELS[segment];
    if (!label && idx > 0 && segments[0] === 'services' && /^\d+$/.test(segment)) {
      label = services.find((s) => s.id === Number(segment))?.name ?? `#${segment}`;
    }
    trail.push({ label: label ?? decodeURIComponent(segment), href });
  });
  return trail;
}
