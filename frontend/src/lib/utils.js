import { clsx } from 'clsx';
import { twMerge } from 'tailwind-merge';
import { cubicOut } from 'svelte/easing';
import { token } from './store/auth.js';
import { dev } from '$app/environment';
import { get } from 'svelte/store';

/** @param {...string} inputs tailwind classes to merge. */
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
    headers = { ...headers, "Authorization": `Bearer ${get(token)}`}
  }
  options = { ...options, headers };
  return fetch(url, options);
}

export const flyAndScale = (node, params = { y: -8, x: 0, start: 0.95, duration: 150 }) => {
  const style = getComputedStyle(node);
  const transform = style.transform === 'none' ? '' : style.transform;

  const scaleConversion = (valueA, scaleA, scaleB) => {
    const [minA, maxA] = scaleA;
    const [minB, maxB] = scaleB;

    const percentage = (valueA - minA) / (maxA - minA);
    const valueB = percentage * (maxB - minB) + minB;

    return valueB;
  };

  const styleToString = (style) => {
    return Object.keys(style).reduce((str, key) => {
      if (style[key] === undefined) return str;
      return str + `${key}:${style[key]};`;
    }, '');
  };

  return {
    duration: params.duration ?? 200,
    delay: 0,
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