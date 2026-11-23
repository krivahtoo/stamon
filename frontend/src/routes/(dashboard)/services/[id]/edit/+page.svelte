<script>
  import { onMount, untrack } from 'svelte';
  import { goto } from '$app/navigation';
  import { toast } from 'svelte-sonner';

  import { PageHeader } from '$lib/components/dashboard/index.js';
  import FormFooter from '$lib/components/service-form/form-footer.svelte';
  import FormSections from '$lib/components/service-form/form-sections.svelte';
  import { editPayload, fromService, validate } from '$lib/components/service-form/service-form.js';
  import { cfetch } from '$lib/utils.js';

  /**
   * @typedef {Object} Props
   * @property {import('./$types').PageData} data
   */

  /** @type {Props} */
  let { data } = $props();

  let service = $derived(data.service);
  let model = $state(untrack(() => fromService(data.service)));
  let saving = $state(false);
  // Saving replaces the service's channels, so it stays off until they load.
  let channelsLoaded = $state(false);

  let detail = $derived(`/services/${service.id}`);

  onMount(async () => {
    try {
      const res = await cfetch(`/services/${service.id}/channels`);
      if (!res.ok) throw new Error(`${res.status}`);
      model.channel_ids = (await res.json()).channel_ids;
      channelsLoaded = true;
    } catch (e) {
      toast.error(`Could not load alert channels: ${e instanceof Error ? e.message : e}`);
    }
  });

  /**
   * @param {string} path
   * @param {unknown} body
   */
  async function put(path, body) {
    const res = await cfetch(path, {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      credentials: 'same-origin',
      body: JSON.stringify(body)
    });
    if (!res.ok) {
      const data = await res.json().catch(() => null);
      throw new Error(data?.message ?? data?.error ?? res.statusText);
    }
  }

  /** @param {SubmitEvent} event */
  async function save(event) {
    event.preventDefault();
    const invalid = validate(model);
    if (invalid) {
      toast.error(invalid);
      return;
    }

    let payload;
    try {
      payload = editPayload(model);
    } catch (e) {
      toast.error(`${e instanceof Error ? e.message : e}`);
      return;
    }

    saving = true;
    try {
      await put(detail, payload);
      await put(`${detail}/channels`, { channel_ids: model.channel_ids });
      toast.success(`Service "${model.name}" saved`);
      await goto(detail, { invalidateAll: true });
    } catch (e) {
      toast.error(`Error: ${e instanceof Error ? e.message : e}`);
    } finally {
      saving = false;
    }
  }
</script>

<form id="edit-service" class="mx-auto flex w-full max-w-3xl flex-col gap-5" onsubmit={save}>
  <PageHeader
    label="Services"
    title="Edit service"
    description={`Change how ${service.name} is checked and who hears about it.`}
  />

  <FormSections bind:model lockedType />
</form>

<FormFooter
  form="edit-service"
  note="Changes apply from the next check."
  submitLabel="Save changes"
  disabled={saving || !channelsLoaded}
  oncancel={() => goto(detail)}
/>
