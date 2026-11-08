import LayoutGrid from '@lucide/svelte/icons/layout-grid';
import Server from '@lucide/svelte/icons/server';
import Wrench from '@lucide/svelte/icons/wrench';
import Bell from '@lucide/svelte/icons/bell';
import Globe from '@lucide/svelte/icons/globe';
import Users from '@lucide/svelte/icons/users';
import Settings from '@lucide/svelte/icons/settings';

/**
 * @typedef {Object} NavItem
 * @property {string} name - Name to show in the sidebar
 * @property {string} path - Navigation item path
 * @property {import('svelte').Component} icon - Navigation item icon
 * @property {string[]} [roles] - Roles that see the item; everyone when unset
 */

/**
 * @typedef {Object} NavGroup
 * @property {string} label - Group heading
 * @property {NavItem[]} items - Items in the group
 */

// The final '/' in each path is important for highlighting
// the current active nav item.
/** @type {NavGroup[]} */
export const navGroups = [
  {
    label: 'Infrastructure',
    items: [
      { name: 'Dashboard', path: '/', icon: LayoutGrid },
      { name: 'Services', path: '/services/', icon: Server },
      { name: 'Maintenance', path: '/maintenance/', icon: Wrench },
      { name: 'Notifications', path: '/notifications/', icon: Bell },
      { name: 'Status Pages', path: '/status-pages/', icon: Globe }
    ]
  },
  {
    label: 'Management',
    items: [
      { name: 'Users', path: '/users/', icon: Users, roles: ['admin'] },
      { name: 'Settings', path: '/settings/', icon: Settings }
    ]
  }
];

/**
 * The groups and items the given role may see. Empty groups are dropped.
 * @param {NavGroup[]} groups
 * @param {string | undefined} role
 * @returns {NavGroup[]}
 */
export function visibleGroups(groups, role) {
  return groups
    .map((group) => ({
      ...group,
      items: group.items.filter((item) => !item.roles || (!!role && item.roles.includes(role)))
    }))
    .filter((group) => group.items.length > 0);
}

/**
 * Whether the nav item at `path` is the current page or one of its children.
 * @param {string} path
 * @param {string | undefined} pathname
 */
export function isActive(path, pathname) {
  return pathname === path || (path !== '/' && !!pathname?.startsWith(path));
}
