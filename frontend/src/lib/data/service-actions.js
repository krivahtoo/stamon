import { toast } from 'svelte-sonner';
import { cfetch } from '$lib/utils.js';

/**
 * @param {string} path
 * @param {RequestInit} options
 */
async function request(path, options) {
  const res = await cfetch(path, { credentials: 'same-origin', ...options });
  if (!res.ok) {
    const body = await res.json().catch(() => null);
    throw body?.message ?? body?.error ?? res.statusText;
  }
  return res.json();
}

/**
 * Pause a service, or resume it when it is paused, with a toast.
 * @param {{ id: number, name: string, active: boolean }} service
 * @returns {Promise<boolean>} Whether the service is now active.
 */
export async function togglePause(service) {
  const pausing = service.active;
  const promise = request(`/services/${service.id}`, {
    method: 'PUT',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ active: !service.active })
  });
  toast.promise(promise, {
    loading: 'Loading...',
    success: `Service "${service.name}" ${pausing ? 'paused' : 'resumed'}`,
    error: (e) => `Error: ${e}`
  });
  await promise;
  return !pausing;
}

/**
 * Delete a service, with a toast.
 * @param {{ id: number, name: string }} service
 * @returns {Promise<void>}
 */
export async function deleteService(service) {
  const promise = request(`/services/${service.id}`, { method: 'DELETE' });
  toast.promise(promise, {
    loading: 'Loading...',
    success: `Service "${service.name}" deleted`,
    error: (e) => `Error: ${e}`
  });
  await promise;
}
