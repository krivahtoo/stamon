import { dev } from '$app/environment';
import { cfetch } from '$lib/utils.js';
import { toast } from 'svelte-sonner';
import services from './services.js';
import stats from './stats.js';
import notifications from './notifications.js';

export const connect = () => {
  //
  cfetch('/services').then(async (res) => {
    if (res.ok) {
      const data = await res.json();
      // console.log(data);
      services.set(data.services);
    }
  });

  cfetch('/stats').then(async (res) => {
    if (res.ok) {
      const data = await res.json();
      // console.log(data);
      stats.set(data.stats);
    }
  });

  // Create a new websocket
  const ws = new WebSocket(dev ? "ws://localhost:3000/ws" : `ws://${location.host}/ws`);

  ws.addEventListener("message", (message) => {
    // console.log(message);
    // Parse the incoming message here
    const data = JSON.parse(message.data);

    if (data.type == "notification") {
      /**
       * @typedef {Object} Notification
       * @property {string} title - Title of the notification.
       * @property {string} message - Content of the notification.
       * @property {string} level - Notification level.
       */

      /** @type {Notification} */
      const noti = data.value;
      // @ts-ignore
      toast[noti.level](noti.title, {
        description: noti.message
      });
      notifications.update((notifications) => ([
        {
          beenRead: false,
          title: noti.title,
          message: noti.message,
        },
        ...notifications
      ]));
    }
  });

  ws.addEventListener("close", () => {
    // Reconnect
    // connect();
    toast.error("Lost connection to the server", {
      description: "Please refresh the page to reconnect."
    })
  });
};