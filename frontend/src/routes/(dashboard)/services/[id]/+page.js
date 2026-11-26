import { error } from '@sveltejs/kit';
import { cfetch } from '$lib/utils.js';
import { goto } from '$app/navigation';

/** @type {import('./$types').PageLoad} */
export async function load({ params }) {
  try {
    const res = await cfetch(`/services/${params.id}`);

    if (res.ok) {
      const data = await res.json();
      return { service: data.service };
    } else if (res.status === 401) {
      goto('/login');
    } else if (res.status === 404) {
      error(404, `Service with id "${params.id}" not found`);
    }

    console.log(res);
    error(res.status, res.statusText);
  } catch (e) {
    console.log(e);
    error(500, `${e}`);
  }
}

// this page has dynamic data so don't prerender
export const prerender = false;

// These loads call the backend through the relative /api URL, which only the
// browser can reach (the dev proxy / reverse proxy), so they must not run on the server.
export const ssr = false;
