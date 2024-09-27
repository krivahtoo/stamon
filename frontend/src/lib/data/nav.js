import Home from 'lucide-svelte/icons/home';
import Layers3 from 'lucide-svelte/icons/layers-3';
import Users from 'lucide-svelte/icons/users';
import Monitor from 'lucide-svelte/icons/monitor';
import MessageSquareText from 'lucide-svelte/icons/message-square-text';

export const navItems = [
  {
    name: 'Dashboard',
    path: '/',
    icon: Home
  },
  {
    name: 'Services',
    // final '/' is important for hightlighting
    // current active navitem
    path: '/services/',
    icon: Layers3
  },
  {
    name: 'Status Pages',
    path: '/status-pages/',
    icon: Monitor
  },
  {
    name: 'Notification Providers',
    path: '/notifications/',
    icon: MessageSquareText
  },
  {
    name: 'Team',
    path: '/users/',
    icon: Users
  }
];
