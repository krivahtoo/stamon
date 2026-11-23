<script>
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { toast } from 'svelte-sonner';

  import { PageHeader } from '$lib/components/dashboard/index.js';
  import FormFooter from '$lib/components/service-form/form-footer.svelte';
  import FormSections from '$lib/components/service-form/form-sections.svelte';
  import {
    createPayload,
    emptyService,
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

  <FormSections bind:model />
</form>

<FormFooter
  form="new-service"
  note="Stamon starts checking as soon as you save."
  submitLabel="Create service"
  disabled={saving}
  oncancel={() => history.back()}
/>
