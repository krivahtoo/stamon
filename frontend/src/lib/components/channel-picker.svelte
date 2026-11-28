<script>
  import { onMount } from 'svelte';
  import BellRing from '@lucide/svelte/icons/bell-ring';
  import Mail from '@lucide/svelte/icons/mail';
  import MessageSquare from '@lucide/svelte/icons/message-square';
  import Send from '@lucide/svelte/icons/send';
  import Webhook from '@lucide/svelte/icons/webhook';

  import { Checkbox } from '$lib/components/ui/checkbox/index.js';
  import { cfetch } from '$lib/utils.js';

  /**
   * @typedef {Object} Props
   * @property {number[]} [selected] - Ids of the channels picked for the service.
   * @property {boolean} [disabled]
   */

  /** @type {Props} */
  let { selected = $bindable([]), disabled = false } = $props();

  /** @type {{ id: number, name: string, active: boolean, apply_to_all: boolean, channel_type: string }[]} */
  let channels = $state([]);
  let loaded = $state(false);

  /** @type {Record<string, { label: string, icon: typeof Mail }>} */
  const types = {
    webhook: { label: 'Webhook', icon: Webhook },
    slack: { label: 'Slack', icon: MessageSquare },
    discord: { label: 'Discord', icon: MessageSquare },
    telegram: { label: 'Telegram', icon: Send },
    ntfy: { label: 'ntfy', icon: BellRing },
    email: { label: 'Email (SMTP)', icon: Mail }
  };

  onMount(async () => {
    const res = await cfetch('/channels', { credentials: 'same-origin' });
    if (res.ok) channels = (await res.json()).channels;
    loaded = true;
  });

  let anyEverywhere = $derived(channels.some((c) => c.apply_to_all));

  /**
   * @param {number} id
   * @param {boolean} checked
   */
  function toggle(id, checked) {
    selected = checked ? [...selected, id] : selected.filter((s) => s !== id);
  }
</script>

<div class="flex flex-col gap-1.5">
  {#if loaded && channels.length === 0}
    <p class="text-sm text-muted-foreground">
      No channels yet. An admin can add them under Notifications.
    </p>
  {:else if channels.length}
    <ul class="divide-y rounded-lg border">
      {#each channels as channel (channel.id)}
        {@const type = types[channel.channel_type]}
        {@const Icon = type?.icon ?? BellRing}
        <li>
          <label
            for={`channel-${channel.id}`}
            class="flex min-h-11 items-center gap-3 px-3.5 py-2 text-sm has-[:disabled]:cursor-default"
          >
            <!-- Channels for every service always alert, so they can't be unpicked. -->
            <Checkbox
              id={`channel-${channel.id}`}
              checked={channel.apply_to_all || selected.includes(channel.id)}
              onCheckedChange={(/** @type {boolean | 'indeterminate'} */ checked) =>
                toggle(channel.id, checked === true)}
              disabled={disabled || channel.apply_to_all}
            />
            <Icon class="size-4 shrink-0 text-muted-foreground" />
            <span class="min-w-0 truncate font-semibold">{channel.name}</span>
            <span class="truncate text-muted-foreground">
              {type?.label ?? channel.channel_type}{channel.active ? '' : ' · paused'}
            </span>
            {#if channel.apply_to_all}
              <span
                class="ml-auto shrink-0 rounded-md border bg-surface px-1.5 py-0.5 text-xs text-muted-foreground"
              >
                All services
              </span>
            {/if}
          </label>
        </li>
      {/each}
    </ul>
    {#if anyEverywhere}
      <p class="text-xs text-muted-foreground">Channels marked All services always alert.</p>
    {/if}
  {/if}
</div>
