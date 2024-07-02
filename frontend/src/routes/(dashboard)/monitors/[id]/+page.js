import { error } from '@sveltejs/kit';

/** @type {import('./$types').PageLoad} */
export function load({ params }) {
  if (parseInt(params.id) === 1) {
    return {
      id: 1,
      active: true,
      name: 'Google DNS',
      url: '8.8.8.8',
      monitor_type: 'ping',
      retry: 3,
      retry_interval: 60
    };
  }

  error(404, `Monitor with id "${params.id}" not found`);
}

// this page has dynamic data so don't prerender
export const prerender = false;