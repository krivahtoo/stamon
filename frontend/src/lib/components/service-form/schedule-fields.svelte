<script>
  import { Input } from '$lib/components/ui/input/index.js';
  import { Switch } from '$lib/components/ui/switch/index.js';
  import FormField from './form-field.svelte';
  import SecondsInput from './seconds-input.svelte';

  /**
   * Interval, timeout, retries and invert.
   * @typedef {Object} Props
   * @property {import('./service-form.js').ServiceModel} model
   */

  /** @type {Props} */
  let { model = $bindable() } = $props();
</script>

<div class="flex flex-col gap-4">
  <div class="grid grid-cols-2 gap-4">
    <FormField label="Interval" id="interval">
      <SecondsInput id="interval" bind:value={model.interval} min="30" max="86400" required />
    </FormField>
    <!-- Push monitors aren't checked, so they have no timeout. -->
    {#if model.service_type !== 'push'}
      <FormField label="Timeout" id="timeout">
        <SecondsInput id="timeout" bind:value={model.timeout} min="1" max="60" required />
      </FormField>
    {/if}
  </div>
  <div class="flex flex-col gap-1.5">
    <div class="grid grid-cols-2 gap-4">
      <FormField label="Retries" id="retry">
        <Input id="retry" type="number" bind:value={model.retry} min="0" required />
      </FormField>
      <FormField label="Retry interval" id="retry_interval">
        <SecondsInput id="retry_interval" bind:value={model.retry_interval} min="1" required />
      </FormField>
    </div>
    <p class="text-xs text-muted-foreground">Failures to allow before the service is down.</p>
  </div>
  <div class="flex items-center justify-between gap-4 rounded-lg border px-3.5 py-3">
    <label for="invert" class="flex flex-col">
      <span class="text-sm font-semibold">Invert check</span>
      <span class="text-xs text-muted-foreground">
        Report the service as down when the check succeeds.
      </span>
    </label>
    <Switch id="invert" bind:checked={model.invert} />
  </div>
</div>
