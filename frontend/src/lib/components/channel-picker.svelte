<script>
  import { onMount } from 'svelte';

  import { Checkbox } from '$lib/components/ui/checkbox/index.js';
  import { Label } from '$lib/components/ui/label/index.js';
  import { cfetch } from '$lib/utils.js';

  /**
   * Ids of the channels picked for the service.
   * @type {number[]}
   */
  export let selected = [];
  export let disabled = false;

  /** @type {{ id: number, name: string, active: boolean, apply_to_all: boolean, channel_type: string }[]} */
  let channels = [];
  let loaded = false;

  onMount(async () => {
    const res = await cfetch('/channels', { credentials: 'same-origin' });
    if (res.ok) channels = (await res.json()).channels;
    loaded = true;
  });

  $: pickable = channels.filter((c) => !c.apply_to_all);
  $: everywhere = channels.filter((c) => c.apply_to_all && c.active);

  /**
   * @param {number} id
   * @param {boolean} checked
   */
  function toggle(id, checked) {
    selected = checked ? [...selected, id] : selected.filter((s) => s !== id);
  }
</script>

<div class="space-y-2">
  {#if loaded && channels.length === 0}
    <p class="text-sm text-muted-foreground">
      No channels yet. An admin can add them under Notification Providers.
    </p>
  {/if}
  {#each pickable as channel (channel.id)}
    <div class="flex items-center gap-2">
      <Checkbox
        id={`channel-${channel.id}`}
        checked={selected.includes(channel.id)}
        onCheckedChange={(/** @type {boolean | 'indeterminate'} */ checked) =>
          toggle(channel.id, checked === true)}
        {disabled}
      />
      <Label for={`channel-${channel.id}`} class="font-normal">
        {channel.name}
        <span class="text-muted-foreground"
          >({channel.channel_type}{channel.active ? '' : ', paused'})</span
        >
      </Label>
    </div>
  {/each}
  {#if everywhere.length > 0}
    <p class="text-xs text-muted-foreground">
      Always notified: {everywhere.map((c) => c.name).join(', ')}
    </p>
  {/if}
</div>
