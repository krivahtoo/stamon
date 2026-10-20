import { clsx } from 'clsx';
import { twMerge } from 'tailwind-merge';
import { cubicOut } from 'svelte/easing';
import { token } from './store/auth.js';
import user from './store/user.js';
import { dev } from '$app/environment';
import { get } from 'svelte/store';

/** @param {...import('clsx').ClassValue} inputs tailwind classes to merge. */
export function cn(...inputs) {
  return twMerge(clsx(inputs));
}

/**
 * @param {string} path
 * @param {RequestInit} options
 * @returns {Promise<Response>}
 */
export function cfetch(path, options = { credentials: 'same-origin' }) {
  const endpoint = dev ? 'http://0.0.0.0:3000/api' : '/api';
  const url = `${endpoint}${path}`;
  let headers = { ...options?.headers };
  if (dev && get(token)) {
    headers = { ...headers, Authorization: `Bearer ${get(token)}` };
  }
  options = { ...options, headers };
  return fetch(url, options);
}

/**
 * A date and time in the signed-in user's timezone, or the browser's when they
 * haven't picked one.
 * @param {string | number | Date} time
 * @returns {string}
 */
export function formatDateTime(time) {
  const timeZone = get(user)?.timezone ?? undefined;
  try {
    return new Date(time).toLocaleString(undefined, { timeZone });
  } catch {
    // An unknown timezone name falls back to the browser's.
    return new Date(time).toLocaleString();
  }
}

/**
 * The URL a push monitor's service calls to send a heartbeat.
 * @param {string} pushToken
 * @returns {string}
 */
export function pushUrl(pushToken) {
  const origin = dev ? 'http://0.0.0.0:3000' : window.location.origin;
  return `${origin}/api/push/${pushToken}`;
}

/**
 * @param {Element} node
 * @param {{ y?: number, x?: number, start?: number, duration?: number }} [params]
 * @returns {import('svelte/transition').TransitionConfig}
 */
export const flyAndScale = (node, params = { y: -8, x: 0, start: 0.95, duration: 150 }) => {
  const style = getComputedStyle(node);
  const transform = style.transform === 'none' ? '' : style.transform;

  /**
   * @param {number} valueA
   * @param {[number, number]} scaleA
   * @param {[number, number]} scaleB
   */
  const scaleConversion = (valueA, scaleA, scaleB) => {
    const [minA, maxA] = scaleA;
    const [minB, maxB] = scaleB;

    const percentage = (valueA - minA) / (maxA - minA);
    const valueB = percentage * (maxB - minB) + minB;

    return valueB;
  };

  /** @param {Record<string, number | string | undefined>} style */
  const styleToString = (style) => {
    return Object.keys(style).reduce((str, key) => {
      if (style[key] === undefined) return str;
      return str + `${key}:${style[key]};`;
    }, '');
  };

  return {
    duration: params.duration ?? 200,
    delay: 0,
    /** @param {number} t */
    css: (t) => {
      const y = scaleConversion(t, [0, 1], [params.y ?? 5, 0]);
      const x = scaleConversion(t, [0, 1], [params.x ?? 0, 0]);
      const scale = scaleConversion(t, [0, 1], [params.start ?? 0.95, 1]);

      return styleToString({
        transform: `${transform} translate3d(${x}px, ${y}px, 0) scale(${scale})`,
        opacity: t
      });
    },
    easing: cubicOut
  };
};
