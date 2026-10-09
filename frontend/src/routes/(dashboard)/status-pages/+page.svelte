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
   * @typedef {Object} StatusPage
   * @property {number} id
   * @property {string} slug
   * @property {string} title
   * @property {string | null} description
   * @property {boolean} published
   * @property {number[]} service_ids
   */

  /** @type {StatusPage[]} */
  let pages = [];
  /** @type {{ id: number, name: string }[]} */
  let services = [];
  let loading = true;

  let formOpen = false;
  /** @type {StatusPage | null} */
  let editing = null;
  let form = emptyForm();
  /** Fill the slug from the title until it's edited by hand. */
  let slugTouched = false;
  /** @type {StatusPage | null} */
  let deleting = null;

  $: canEdit = $user?.role === 'admin' || $user?.role === 'editor';
  $: if (!slugTouched) form.slug = slugify(form.title);

  function emptyForm() {
    return {
      title: '',
      slug: '',
      description: '',
      published: false,
      /** @type {number[]} */
      service_ids: []
    };
  }

  /** @param {string} text */
  function slugify(text) {
    return text
      .toLowerCase()
      .replace(/[^a-z0-9]+/g, '-')
      .replace(/^-+|-+$/g, '')
      .slice(0, 64);
  }

  /** @param {string} slug */
  const publicPath = (slug) => `/status/${slug}/`;

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
      const [pageData, serviceData] = await Promise.all([
        request('/status-pages'),
        request('/services')
      ]);
      pages = pageData.status_pages;
      services = serviceData.services;
    } catch (e) {
      toast.error(`Could not load status pages: ${errorText(e)}`);
    } finally {
      loading = false;
    }
  }

  onMount(load);

  function add() {
    editing = null;
    form = emptyForm();
    slugTouched = false;
    formOpen = true;
  }

  /** @param {StatusPage} page */
  function edit(page) {
    editing = page;
    form = {
      title: page.title,
      slug: page.slug,
      description: page.description ?? '',
      published: page.published,
      service_ids: [...page.service_ids]
    };
    slugTouched = true;
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
    try {
      await request(editing ? `/status-pages/${editing.id}` : '/status-pages', {
        method: editing ? 'PUT' : 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(form)
      });
      toast.success(editing ? 'Status page updated' : 'Status page created');
      formOpen = false;
      await load();
    } catch (e) {
      toast.error(`Error: ${errorText(e)}`);
    }
  }

  async function confirmDelete() {
    if (!deleting) return;
    const page = deleting;
    deleting = null;
    try {
      await request(`/status-pages/${page.id}`, { method: 'DELETE' });
      toast.success(`"${page.title}" deleted`);
    } catch (e) {
      toast.error(`Error: ${errorText(e)}`);
    }
    await load();
  }
</script>

<div class="flex items-center justify-between">
  <div class="space-y-0.5">
    <h2 class="text-2xl font-bold tracking-tight">Status Pages</h2>
    <p class="text-muted-foreground">Public pages showing the status of chosen services.</p>
  </div>
  {#if canEdit}
    <Button on:click={add}>New status page</Button>
  {/if}
</div>

{#if !loading && pages.length === 0}
  <section
    class="flex min-h-[200px] items-center justify-center rounded-lg border border-dashed shadow-sm"
  >
    <div class="flex flex-col items-center gap-1 p-6 text-center">
      <h3 class="text-xl font-bold tracking-tight">You have no status pages</h3>
      <p class="text-sm text-muted-foreground">
        Share a page so users can see what's up without an account.
      </p>
      {#if canEdit}
        <Button class="mt-4" on:click={add}>New status page</Button>
      {/if}
    </div>
  </section>
{:else}
  <div class="grid gap-4 md:grid-cols-2 xl:grid-cols-3">
    {#each pages as page (page.id)}
      <Card.Root>
        <Card.Header class="pb-2">
          <div class="flex items-start justify-between gap-2">
            <Card.Title class="text-base">{page.title}</Card.Title>
            <Badge variant={page.published ? 'default' : 'outline'}>
              {page.published ? 'Published' : 'Draft'}
            </Badge>
          </div>
          <Card.Description>
            {#if page.published}
              <a class="underline" href={publicPath(page.slug)} target="_blank" rel="noreferrer"
                >{publicPath(page.slug)}</a
              >
            {:else}
              {publicPath(page.slug)} (publish to share)
            {/if}
          </Card.Description>
        </Card.Header>
        <Card.Content class="text-sm text-muted-foreground">
          {page.service_ids.length} service{page.service_ids.length === 1 ? '' : 's'}
        </Card.Content>
        {#if canEdit}
          <Card.Footer class="gap-2">
            <Button size="sm" variant="outline" on:click={() => edit(page)}>Edit</Button>
            <Button
              size="sm"
              variant="ghost"
              class="ml-auto text-destructive"
              on:click={() => (deleting = page)}>Delete</Button
            >
          </Card.Footer>
        {/if}
      </Card.Root>
    {/each}
  </div>
{/if}

<Dialog.Root bind:open={formOpen}>
  <Dialog.Content class="max-h-[90vh] overflow-y-auto sm:max-w-[520px]">
    <Dialog.Header>
      <Dialog.Title>{editing ? 'Edit status page' : 'New status page'}</Dialog.Title>
      <Dialog.Description>
        Visitors see service names, statuses and uptime, never URLs or hosts.
      </Dialog.Description>
    </Dialog.Header>
    <form id="status-page-form" class="grid gap-4 py-2" on:submit|preventDefault={save}>
      <div class="grid gap-2">
        <Label for="title">Title</Label>
        <Input id="title" bind:value={form.title} placeholder="e.g. Acme Status" required />
      </div>
      <div class="grid gap-2">
        <Label for="slug">Slug</Label>
        <Input
          id="slug"
          bind:value={form.slug}
          on:input={() => (slugTouched = true)}
          placeholder="acme"
          pattern="[a-z0-9]+(-[a-z0-9]+)*"
          required
        />
        <p class="text-xs text-muted-foreground">Public address: {publicPath(form.slug || '…')}</p>
      </div>
      <div class="grid gap-2">
        <Label for="description">Description</Label>
        <Textarea id="description" bind:value={form.description} />
      </div>
      <div class="flex items-center justify-between gap-4">
        <div>
          <Label for="published">Published</Label>
          <p class="text-xs text-muted-foreground">Only published pages can be visited.</p>
        </div>
        <Switch id="published" bind:checked={form.published} />
      </div>
      <div class="grid gap-2">
        <Label>Services</Label>
        <p class="text-xs text-muted-foreground">Shown in the order you pick them.</p>
        <div class="grid max-h-48 gap-2 overflow-y-auto">
          {#each services as service (service.id)}
            <div class="flex items-center gap-2">
              <Checkbox
                id={`page-service-${service.id}`}
                checked={form.service_ids.includes(service.id)}
                onCheckedChange={(/** @type {boolean | 'indeterminate'} */ checked) =>
                  toggleService(service.id, checked === true)}
              />
              <Label for={`page-service-${service.id}`} class="font-normal">
                {service.name}
                {#if form.service_ids.includes(service.id)}
                  <span class="text-muted-foreground"
                    >#{form.service_ids.indexOf(service.id) + 1}</span
                  >
                {/if}
              </Label>
            </div>
          {/each}
        </div>
      </div>
    </form>
    <Dialog.Footer>
      <Button variant="outline" on:click={() => (formOpen = false)}>Cancel</Button>
      <Button type="submit" form="status-page-form">{editing ? 'Save' : 'Create'}</Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>

<Dialog.Root open={deleting !== null} onOpenChange={(open) => !open && (deleting = null)}>
  <Dialog.Content class="sm:max-w-[425px]">
    <Dialog.Header>
      <Dialog.Title>Delete status page</Dialog.Title>
      <Dialog.Description>
        Delete "{deleting?.title}"? Its public address stops working.
      </Dialog.Description>
    </Dialog.Header>
    <Dialog.Footer>
      <Button variant="outline" on:click={() => (deleting = null)}>Cancel</Button>
      <Button variant="destructive" on:click={confirmDelete}>Yes, delete</Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>
