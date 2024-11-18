import { dev } from '$app/environment';
import { websocket } from '@sveu/browser';
import { toast } from 'svelte-sonner';
import notifications from './notifications.js';

const toastId = 3000;

/** @type {import('@sveu/browser').WebSocketOptions} */
const options = {
  immediate: false,
  autoReconnect: {
    delay: 30,
    retries: 5,
    onFailed: () => {
      toast.error('Failed to reconnect to websocket!', {
        id: toastId,
        description: 'Please reload the page and try again'
      });
    }
  },
  onConnected: () => {
    toast.success('Websocket connected!', {
      id: toastId
    });
  },
  onMessage: (_, ev) => {
    // console.log(ev.data);
    // Parse the incoming message here
    const data = JSON.parse(ev.data);

    if (data.type == 'notification') {
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
      notifications.update((notifications) => [
        {
          beenRead: false,
          title: noti.title,
          message: noti.message
        },
        ...notifications
      ]);
    }
  },
  onDisconnected: () => {
    toast.warning('Websocket disconnected!', {
      id: toastId,
      description: 'Reconnecting in 30 seconds...'
      // dismissible: false
    });
    let now = Date.now();
    let interval = setInterval(() => {
      toast.info(`Reconnecting in ${30 - Math.ceil((Date.now() - now) / 1000)} seconds...`, {
        id: toastId,
        description: ''
      });
      if (Date.now() - now > 29000) {
        clearInterval(interval);
        toast.loading('Reconnecting...', {
          id: toastId,
          duration: 3000
          // dismissible: true
        });
      }
    }, 500);
  },
  onError: (ws, err) => {
    console.log(err);
  }
};

// Create a new websocket
export const ws = websocket(dev ? 'ws://localhost:3000/ws' : `ws://${location.host}/ws`, options);
