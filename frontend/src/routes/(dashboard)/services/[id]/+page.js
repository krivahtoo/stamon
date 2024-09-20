import { error } from '@sveltejs/kit';
import { cfetch } from '$lib/utils.js';

/** @type {import('./$types').PageLoad} */
export async function load({ params }) {
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
  }

  console.log(res);

  error(404, `Service with id "${params.id}" not found`);
}

// this page has dynamic data so don't prerender
export const prerender = false;