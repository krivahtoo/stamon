import { error } from '@sveltejs/kit';
import { cfetch } from '$lib/utils.js';

/** @type {import('./$types').PageLoad} */
export async function load({ params }) {
  const res = await cfetch(`/status/${params.slug}`);
  if (res.status === 404) {
    error(404, 'Status page not found');
  }
  if (!res.ok) {
    error(res.status, res.statusText);
  }
  return { page: await res.json() };
}

// Pages are created at runtime, so they can't be prerendered.
export const prerender = false;

// These loads call the backend through the relative /api URL, which only the
// browser can reach (the dev proxy / reverse proxy), so they must not run on the server.
export const ssr = false;
