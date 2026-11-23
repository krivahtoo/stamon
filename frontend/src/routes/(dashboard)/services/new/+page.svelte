<script>
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { toast } from 'svelte-sonner';

  import { Button } from '$lib/components/ui/button/index.js';
  import { PageHeader } from '$lib/components/dashboard/index.js';
  import ChannelPicker from '$lib/components/channel-picker.svelte';
  import TagInput from '$lib/components/tag-input.svelte';
  import CheckTypeGrid from '$lib/components/service-form/check-type-grid.svelte';
  import CheckFields from '$lib/components/service-form/check-fields.svelte';
  import FormField from '$lib/components/service-form/form-field.svelte';
  import FormSection from '$lib/components/service-form/form-section.svelte';
  import ScheduleFields from '$lib/components/service-form/schedule-fields.svelte';
  import {
    MAX_TAGS,
    MAX_TAG_LEN,
    createPayload,
    emptyService,
    kindLabel,
    validate
  } from '$lib/components/service-form/service-form.js';
  import { cfetch } from '$lib/utils.js';

  /** Scheduling defaults, replaced by the ones saved in Settings once loaded. */
  let defaults = { retry: 1, retry_interval: 30, interval: 60, timeout: 20 };

  let model = $state(emptyService(defaults));
  let saving = $state(false);

  onMount(async () => {
    const res = await cfetch('/settings').catch(() => null);
    if (!res?.ok) return;
    const { settings } = await res.json();
    const loaded = {
      retry: settings.default_retry,
      retry_interval: settings.default_retry_interval,
      interval: settings.default_interval,
      timeout: settings.default_timeout
    };
    // Keep values the user already typed or restored from a snapshot.
    for (const key of /** @type {(keyof typeof defaults)[]} */ (Object.keys(loaded))) {
      if (model[key] === defaults[key]) model[key] = loaded[key];
    }
    defaults = loaded;
  });

  /** @type {import('./$types').Snapshot<import('$lib/components/service-form/service-form.js').ServiceModel>} */
  export const snapshot = {
    capture: () => model,
    restore: (value) => (model = value)
  };

  /** @param {SubmitEvent} event */
  async function create(event) {
    event.preventDefault();
    const invalid = validate(model);
    if (invalid) {
      toast.error(invalid);
      return;
    }

    let payload;
    try {
      payload = createPayload(model);
    } catch (e) {
      toast.error(`${e instanceof Error ? e.message : e}`);
      return;
    }

    saving = true;
    try {
      const res = await cfetch('/services', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        credentials: 'same-origin',
        body: JSON.stringify(payload)
      });
      const body = await res.json().catch(() => null);
      if (!res.ok) {
        toast.error(`Error: ${body?.message ?? body?.error ?? res.statusText}`);
        return;
      }
      toast.success(`Service "${model.name}" added`);
      await goto(`/services/${body.id}`);
    } catch (e) {
      toast.error(`Error: ${e instanceof Error ? e.message : e}`);
    } finally {
      saving = false;
    }
  }
</script>

<form id="new-service" class="mx-auto flex w-full max-w-3xl flex-col gap-5" onsubmit={create}>
  <PageHeader
    label="Services"
    title="New service"
    description="Choose what to check, then how often and who to alert."
  />

  <FormSection number={1} title="Check type" description="What Stamon should check.">
    <CheckTypeGrid bind:model />
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
</form>

<!-- In the page flow, so it sticks to the bottom without covering the last fields. -->
<footer class="sticky bottom-0 -mx-4 -mb-8 border-t bg-background px-4 py-3 md:-mx-8 md:px-8">
  <div class="mx-auto flex max-w-3xl items-center gap-2">
    <p class="mr-auto hidden text-sm text-muted-foreground sm:block">
      Stamon starts checking as soon as you save.
    </p>
    <Button variant="outline" class="flex-1 sm:flex-none" onclick={() => history.back()}>
      Cancel
    </Button>
    <Button type="submit" form="new-service" class="flex-1 sm:flex-none" disabled={saving}
      >Create service</Button
    >
  </div>
</footer>
