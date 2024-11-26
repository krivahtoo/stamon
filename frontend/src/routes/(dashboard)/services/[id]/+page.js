import { error } from '@sveltejs/kit';
import { cfetch } from '$lib/utils.js';
import { goto } from '$app/navigation';

/** @type {import('./$types').PageLoad} */
export async function load({ params }) {
  try {
    const res = await cfetch(`/services/${params.id}`);

    if (res.ok) {
      const data = await res.json();

      const res_log = await cfetch(`/services/${params.id}/logs?limit=60`);
      const logs = await res_log.json();
      // console.log(logs);
      return {
        service: data.service,
        logs: logs.logs.reverse()
      };
    } else if (res.status === 403) {
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
