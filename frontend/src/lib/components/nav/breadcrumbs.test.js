import { describe, expect, it } from 'vitest';
import { crumbs } from './breadcrumbs.js';
import { navGroups, visibleGroups } from '$lib/data/nav.js';

/**
 * @param {string} pathname
 * @param {{ id: number, name: string }[]} [services]
 */
const labels = (pathname, services) => crumbs(pathname, services).map((c) => c.label);

describe('crumbs', () => {
  it('names the dashboard', () => {
    expect(labels('/')).toEqual(['Stamon', 'Dashboard']);
  });

  it('names nav pages', () => {
    expect(labels('/services/')).toEqual(['Stamon', 'Services']);
    expect(labels('/status-pages/')).toEqual(['Stamon', 'Status Pages']);
  });

  it('names a service by id', () => {
    const services = [{ id: 3, name: 'API' }];
    expect(labels('/services/3/', services)).toEqual(['Stamon', 'Services', 'API']);
    expect(labels('/services/9/', services)).toEqual(['Stamon', 'Services', '#9']);
  });

  it('names sub pages and links each level', () => {
    expect(crumbs('/services/new/')).toEqual([
      { label: 'Stamon', href: '/' },
      { label: 'Services', href: '/services/' },
      { label: 'New service', href: '/services/new/' }
    ]);
    expect(labels('/settings/appearance/')).toEqual(['Stamon', 'Settings', 'Appearance']);
  });
});

describe('visibleGroups', () => {
  /** @param {string} role */
  const names = (role) => visibleGroups(navGroups, role).flatMap((g) => g.items.map((i) => i.name));

  it('shows Users to admins only', () => {
    expect(names('admin')).toContain('Users');
    expect(names('editor')).not.toContain('Users');
    expect(names('viewer')).not.toContain('Users');
    expect(names('viewer')).toContain('Settings');
  });
});
