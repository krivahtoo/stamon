<script>
  import ChannelPicker from '$lib/components/channel-picker.svelte';
  import TagInput from '$lib/components/tag-input.svelte';
  import CheckFields from './check-fields.svelte';
  import CheckTypeGrid from './check-type-grid.svelte';
  import FormField from './form-field.svelte';
  import FormSection from './form-section.svelte';
  import ScheduleFields from './schedule-fields.svelte';
  import { MAX_TAGS, MAX_TAG_LEN, kindLabel } from './service-form.js';

  /**
   * The four numbered sections of the new and edit service pages.
   * @typedef {Object} Props
   * @property {import('./service-form.js').ServiceModel} model
   * @property {boolean} [lockedType] - Editing: show only the service's type, which can't change.
   */

  /** @type {Props} */
  let { model = $bindable(), lockedType = false } = $props();
</script>

<FormSection
  number={1}
  title="Check type"
  description={lockedType
    ? 'To change the type, create a new service.'
    : 'What Stamon should check.'}
>
  <CheckTypeGrid bind:model locked={lockedType} />
</FormSection>

<FormSection
  number={2}
  title={`${kindLabel(model)} details`}
  description={model.service_type === 'http'
    ? 'Where to send the request and what counts as up.'
    : 'What to check and what counts as up.'}
>
  <CheckFields bind:model />
</FormSection>

<FormSection
  number={3}
  title="Schedule"
  description="How often to check, and how many failures before alerting."
>
  <ScheduleFields bind:model />
</FormSection>

<FormSection
  number={4}
  title="Tags and alerts"
  description="Group the service and choose who hears about it."
>
  <div class="flex flex-col gap-4">
    <FormField label="Tags" id="tags" hint="Comma-separated, e.g. prod, api">
      <TagInput id="tags" bind:tags={model.tags} max={MAX_TAGS} maxLength={MAX_TAG_LEN} />
    </FormField>
    <div class="flex flex-col gap-1.5">
      <h3 class="text-sm font-semibold">Alert channels</h3>
      <ChannelPicker bind:selected={model.channel_ids} />
    </div>
  </div>
</FormSection>
