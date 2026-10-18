<script>
  import { onMount } from 'svelte';
  import { toast } from 'svelte-sonner';

  import { Button } from '$lib/components/ui/button/index.js';
  import { Input } from '$lib/components/ui/input/index.js';
  import { Label } from '$lib/components/ui/label/index.js';
  import demo from '$lib/store/demo.js';
  import user from '$lib/store/user.js';
  import DemoCallout from '../demo-callout.svelte';
  import { errorMessage, request, send } from '../request.js';

  /**
   * @typedef {Object} Settings
   * @property {number} log_retention_days
   * @property {number} default_interval
   * @property {number} default_timeout
   * @property {number} default_retry
   * @property {number} default_retry_interval
   */

  /**
   * @typedef {Object} Instance
   * @property {string} version
   * @property {boolean} demo
   * @property {string | null} data_path
   * @property {number} token_minutes
   */

  /** @type {Settings | null} */
  let saved = null;
  /** @type {Settings | null} */
  let form = null;
  /** @type {Instance | null} */
  let instance = null;
  let saving = false;

  $: readOnly = $demo || $user?.role !== 'admin';
  $: changed = !!form && !!saved && JSON.stringify(form) !== JSON.stringify(saved);

  async function load() {
    try {
      const data = await request('/settings');
      saved = data.settings;
      form = { ...data.settings };
      instance = data.instance;
    } catch (e) {
      toast.error(`Could not load settings: ${errorMessage(e)}`);
    }
  }

  onMount(load);

  async function save() {
    if (!form) return;
    saving = true;
    try {
      /** @type {Settings} */
      const body = {
        log_retention_days: Number(form.log_retention_days),
        default_interval: Number(form.default_interval),
        default_timeout: Number(form.default_timeout),
        default_retry: Number(form.default_retry),
        default_retry_interval: Number(form.default_retry_interval)
      };
      await send('/settings', 'PUT', body);
      toast.success('Settings saved');
      await load();
    } catch (e) {
      toast.error(errorMessage(e));
    } finally {
      saving = false;
    }
  }
</script>

<svelte:head>
  <title>General settings · Stamon</title>
</svelte:head>

<DemoCallout />

{#if form}
  <form on:submit|preventDefault={save}>
    <section class="space-y-4 border-b pb-8">
      <div>
        <h3 class="text-lg font-semibold">1. Data retention</h3>
        <p class="text-sm text-muted-foreground">
          Check logs and alert history older than this are deleted every hour. Use 0 to keep them
          forever.
        </p>
      </div>
      <div class="grid gap-2 sm:max-w-xs">
        <Label for="retention">Keep history for (days)</Label>
        <Input
          id="retention"
          type="number"
          min="0"
          bind:value={form.log_retention_days}
          disabled={readOnly}
          required
        />
      </div>
    </section>

    <section class="space-y-4 border-b py-8">
      <div>
        <h3 class="text-lg font-semibold">2. New service defaults</h3>
        <p class="text-sm text-muted-foreground">
          The values a new service starts with. Existing services keep their own.
        </p>
      </div>
      <div class="grid gap-4 sm:grid-cols-2">
        <div class="grid gap-2">
          <Label for="interval">Check interval (seconds)</Label>
          <Input
            id="interval"
            type="number"
            min={$demo ? 60 : 1}
            bind:value={form.default_interval}
            disabled={readOnly}
            required
          />
        </div>
        <div class="grid gap-2">
          <Label for="timeout">Timeout (seconds)</Label>
          <Input
            id="timeout"
            type="number"
            min="1"
            bind:value={form.default_timeout}
            disabled={readOnly}
            required
          />
        </div>
        <div class="grid gap-2">
          <Label for="retry">Retries before alerting</Label>
          <Input
            id="retry"
            type="number"
            min="0"
            bind:value={form.default_retry}
            disabled={readOnly}
            required
          />
        </div>
        <div class="grid gap-2">
          <Label for="retry-interval">Retry interval (seconds)</Label>
          <Input
            id="retry-interval"
            type="number"
            min="0"
            bind:value={form.default_retry_interval}
            disabled={readOnly}
            required
          />
        </div>
      </div>
    </section>

    {#if instance}
      <section class="space-y-4 py-8">
        <div>
          <h3 class="text-lg font-semibold">3. Instance</h3>
          <p class="text-sm text-muted-foreground">
            Set with environment variables when Stamon starts.
          </p>
        </div>
        <dl class="grid grid-cols-[10rem_1fr] gap-y-2 text-sm">
          <dt class="text-muted-foreground">Version</dt>
          <dd class="font-medium">{instance.version}</dd>
          <dt class="text-muted-foreground">Demo mode</dt>
          <dd class="font-medium">{instance.demo ? 'On' : 'Off'}</dd>
          <dt class="text-muted-foreground">Sign-in lasts</dt>
          <dd class="font-medium">{instance.token_minutes} minutes</dd>
          {#if instance.data_path}
            <dt class="text-muted-foreground">Data directory</dt>
            <dd class="break-all font-mono text-xs leading-5">{instance.data_path}</dd>
          {/if}
        </dl>
      </section>
    {/if}

    {#if !readOnly}
      <div
        class="sticky bottom-0 flex justify-end gap-2 border-t bg-background/95 py-3 backdrop-blur"
      >
        <Button
          type="button"
          variant="outline"
          disabled={!changed || saving}
          on:click={() => saved && (form = { ...saved })}
        >
          Cancel
        </Button>
        <Button type="submit" disabled={!changed || saving}>Save</Button>
      </div>
    {/if}
  </form>
{/if}
