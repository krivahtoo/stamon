import { cfetch } from '$lib/utils.js';

/**
 * Fetch JSON from the API, throwing the server's message on failure.
 * @param {string} path
 * @param {RequestInit} [options]
 */
export async function request(path, options) {
  const res = await cfetch(path, { credentials: 'same-origin', ...options });
  const data = await res.json().catch(() => null);
  if (!res.ok) throw new Error(data?.message ?? data?.error ?? res.statusText);
  return data;
}

/**
 * Send `body` as JSON with `method`.
 * @param {string} path
 * @param {string} method
 * @param {unknown} body
 */
export function send(path, method, body) {
  return request(path, {
    method,
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(body)
  });
}

/** @param {unknown} e */
export const errorMessage = (e) => (e instanceof Error ? e.message : `${e}`);
