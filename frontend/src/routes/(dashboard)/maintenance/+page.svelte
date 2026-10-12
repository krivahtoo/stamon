<script>
  import { onMount } from 'svelte';
  import { toast } from 'svelte-sonner';

  import { Badge } from '$lib/components/ui/badge/index.js';
  import { Button } from '$lib/components/ui/button/index.js';
  import * as Card from '$lib/components/ui/card/index.js';
  import { Checkbox } from '$lib/components/ui/checkbox/index.js';
  import * as Dialog from '$lib/components/ui/dialog/index.js';
  import { Input } from '$lib/components/ui/input/index.js';
  import { Label } from '$lib/components/ui/label/index.js';
  import { Switch } from '$lib/components/ui/switch/index.js';
  import { Textarea } from '$lib/components/ui/textarea/index.js';
  import user from '$lib/store/user.js';
  import { cfetch } from '$lib/utils.js';

  /**
   * @typedef {Object} Window
   * @property {number} id
   * @property {string} title
   * @property {string | null} description
   * @property {string} starts_at
   * @property {string} ends_at
   * @property {boolean} apply_to_all
   * @property {number[]} service_ids
   */

  /** @type {Window[]} */
  let windows = [];
  /** @type {{ id: number, name: string }[]} */
  let services = [];
  let loading = true;
  let now = Date.now();

  let formOpen = false;
  /** @type {Window | null} */
  let editing = null;
  let form = emptyForm();
  /** @type {Window | null} */
  let deleting = null;

  $: canEdit = $user?.role === 'admin' || $user?.role === 'editor';
  $: serviceNames = new Map(services.map((s) => [s.id, s.name]));

  /**
   * A date as the value of a `datetime-local` input, in local time.
   * @param {Date} date
   */
  function toLocalInput(date) {
    const offset = date.getTimezoneOffset() * 60000;
    return new Date(date.getTime() - offset).toISOString().slice(0, 16);
  }

  function emptyForm() {
    const start = new Date(Date.now() + 3600000);
    start.setMinutes(0, 0, 0);
    return {
      title: '',
      description: '',
      starts_at: toLocalInput(start),
      ends_at: toLocalInput(new Date(start.getTime() + 3600000)),
      apply_to_all: false,
      /** @type {number[]} */
      service_ids: []
    };
  }

  /**
   * @param {string} path
   * @param {RequestInit} [options]
   */
  async function request(path, options) {
    const res = await cfetch(path, { credentials: 'same-origin', ...options });
    const data = await res.json().catch(() => null);
    if (!res.ok) throw new Error(data?.message ?? data?.error ?? res.statusText);
    return data;
  }

  /** @param {unknown} e */
  const errorText = (e) => (e instanceof Error ? e.message : `${e}`);

  async function load() {
    try {
      const [maintenance, serviceData] = await Promise.all([
        request('/maintenance'),
        request('/services')
      ]);
      windows = maintenance.maintenance;
      services = serviceData.services;
      now = Date.now();
    } catch (e) {
      toast.error(`Could not load maintenance: ${errorText(e)}`);
    } finally {
      loading = false;
    }
  }

  onMount(load);

  /** @param {Window} window */
  function phase(window) {
    if (new Date(window.ends_at).getTime() <= now) return 'past';
    if (new Date(window.starts_at).getTime() <= now) return 'active';
    return 'upcoming';
  }

  $: active = windows.filter((w) => phase(w) === 'active');
  $: upcoming = windows.filter((w) => phase(w) === 'upcoming').reverse();
  $: past = windows.filter((w) => phase(w) === 'past');

  function add() {
    editing = null;
    form = emptyForm();
    formOpen = true;
  }

  /** @param {Window} window */
  function edit(window) {
    editing = window;
    form = {
      title: window.title,
      description: window.description ?? '',
      starts_at: toLocalInput(new Date(window.starts_at)),
      ends_at: toLocalInput(new Date(window.ends_at)),
      apply_to_all: window.apply_to_all,
      service_ids: [...window.service_ids]
    };
    formOpen = true;
  }

  /**
   * @param {number} id
   * @param {boolean} checked
   */
  function toggleService(id, checked) {
    form.service_ids = checked
      ? [...form.service_ids, id]
      : form.service_ids.filter((s) => s !== id);
  }

  async function save() {
    const body = {
      ...form,
      starts_at: new Date(form.starts_at).toISOString(),
      ends_at: new Date(form.ends_at).toISOString()
    };
    try {
      await request(editing ? `/maintenance/${editing.id}` : '/maintenance', {
        method: editing ? 'PUT' : 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(body)
      });
      toast.success(editing ? 'Maintenance updated' : 'Maintenance scheduled');
      formOpen = false;
      await load();
    } catch (e) {
      toast.error(`Error: ${errorText(e)}`);
    }
  }

  async function confirmDelete() {
    if (!deleting) return;
    const window = deleting;
    deleting = null;
    try {
      await request(`/maintenance/${window.id}`, { method: 'DELETE' });
      toast.success(`"${window.title}" deleted`);
    } catch (e) {
      toast.error(`Error: ${errorText(e)}`);
    }
    await load();
  }

  /** @param {Window} window */
  function coverage(window) {
    if (window.apply_to_all) return 'All services';
    return window.service_ids.map((id) => serviceNames.get(id) ?? `#${id}`).join(', ');
  }

  /** @param {string} time */
  const formatTime = (time) => new Date(time).toLocaleString();
</script>

<div class="flex items-center justify-between">
  <div class="space-y-0.5">
    <h2 class="text-2xl font-bold tracking-tight">Maintenance</h2>
    <p class="text-muted-foreground">
      Planned downtime. Covered services aren't checked and don't send alerts.
    </p>
  </div>
  {#if canEdit}
    <Button on:click={add}>Schedule maintenance</Button>
  {/if}
</div>

{#if !loading && windows.length === 0}
  <section
    class="flex min-h-[200px] items-center justify-center rounded-lg border border-dashed shadow-sm"
  >
    <div class="flex flex-col items-center gap-1 p-6 text-center">
      <h3 class="text-xl font-bold tracking-tight">No maintenance scheduled</h3>
      <p class="text-sm text-muted-foreground">
        Schedule a window before planned work so it doesn't count as downtime.
      </p>
    </div>
  </section>
{/if}

{#each [{ label: 'In progress', items: active, variant: 'default' }, { label: 'Upcoming', items: upcoming, variant: 'secondary' }, { label: 'Past', items: past, variant: 'outline' }] as group}
  {#if group.items.length > 0}
    <section class="space-y-2">
      <h3 class="text-sm font-medium text-muted-foreground">{group.label}</h3>
      <div class="grid gap-4 md:grid-cols-2">
        {#each group.items as window (window.id)}
          <Card.Root class={group.label === 'Past' ? 'opacity-70' : ''}>
            <Card.Header class="pb-2">
              <div class="flex items-start justify-between gap-2">
                <Card.Title class="text-base">{window.title}</Card.Title>
                <Badge variant={group.variant}>{group.label}</Badge>
              </div>
              <Card.Description>
                {formatTime(window.starts_at)} – {formatTime(window.ends_at)}
              </Card.Description>
            </Card.Header>
            <Card.Content class="space-y-1 text-sm">
              <p><span class="text-muted-foreground">Covers:</span> {coverage(window)}</p>
              {#if window.description}
                <p class="text-muted-foreground">{window.description}</p>
              {/if}
            </Card.Content>
            {#if canEdit}
              <Card.Footer class="gap-2">
                <Button size="sm" variant="outline" on:click={() => edit(window)}>Edit</Button>
                <Button
                  size="sm"
                  variant="ghost"
                  class="ml-auto text-destructive"
                  on:click={() => (deleting = window)}>Delete</Button
                >
              </Card.Footer>
            {/if}
          </Card.Root>
        {/each}
      </div>
    </section>
  {/if}
{/each}

<Dialog.Root bind:open={formOpen}>
  <Dialog.Content class="max-h-[90vh] overflow-y-auto sm:max-w-[520px]">
    <Dialog.Header>
      <Dialog.Title>{editing ? 'Edit maintenance' : 'Schedule maintenance'}</Dialog.Title>
      <Dialog.Description>Times are in your local time zone.</Dialog.Description>
    </Dialog.Header>
    <form id="maintenance-form" class="grid gap-4 py-2" on:submit|preventDefault={save}>
      <div class="grid gap-2">
        <Label for="title">Title</Label>
        <Input id="title" bind:value={form.title} placeholder="e.g. Database upgrade" required />
      </div>
      <div class="grid gap-2">
        <Label for="description">Description</Label>
        <Textarea
          id="description"
          bind:value={form.description}
          placeholder="Shown on status pages"
        />
      </div>
      <div class="grid grid-cols-2 gap-2">
        <div class="grid gap-2">
          <Label for="starts_at">Starts</Label>
          <Input id="starts_at" type="datetime-local" bind:value={form.starts_at} required />
        </div>
        <div class="grid gap-2">
          <Label for="ends_at">Ends</Label>
          <Input id="ends_at" type="datetime-local" bind:value={form.ends_at} required />
        </div>
      </div>
      <div class="flex items-center justify-between gap-4">
        <Label for="apply_to_all">All services</Label>
        <Switch id="apply_to_all" bind:checked={form.apply_to_all} />
      </div>
      {#if !form.apply_to_all}
        <div class="grid max-h-48 gap-2 overflow-y-auto">
          {#each services as service (service.id)}
            <div class="flex items-center gap-2">
              <Checkbox
                id={`service-${service.id}`}
                checked={form.service_ids.includes(service.id)}
                onCheckedChange={(/** @type {boolean | 'indeterminate'} */ checked) =>
                  toggleService(service.id, checked === true)}
              />
              <Label for={`service-${service.id}`} class="font-normal">{service.name}</Label>
            </div>
          {/each}
        </div>
      {/if}
    </form>
    <Dialog.Footer>
      <Button variant="outline" on:click={() => (formOpen = false)}>Cancel</Button>
      <Button type="submit" form="maintenance-form">{editing ? 'Save' : 'Schedule'}</Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>

<Dialog.Root open={deleting !== null} onOpenChange={(open) => !open && (deleting = null)}>
  <Dialog.Content class="sm:max-w-[425px]">
    <Dialog.Header>
      <Dialog.Title>Delete maintenance</Dialog.Title>
      <Dialog.Description>
        Delete "{deleting?.title}"? Covered services are checked normally again.
      </Dialog.Description>
    </Dialog.Header>
    <Dialog.Footer>
      <Button variant="outline" on:click={() => (deleting = null)}>Cancel</Button>
      <Button variant="destructive" on:click={confirmDelete}>Yes, delete</Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>
