<script>
  import { toast } from 'svelte-sonner';
  import * as Card from '$lib/components/ui/card/index.js';
  import * as Select from '$lib/components/ui/select/index.js';
  import { Button } from '$lib/components/ui/button/index.js';
  import { Input } from '$lib/components/ui/input/index.js';
  import { Switch } from '$lib/components/ui/switch/index.js';
  import {
    fromService,
    httpMethods,
    recordTypes,
    updatePayload,
    validate
  } from '$lib/components/service-form/service-form.js';
  import { typeLabel } from '$lib/data/checks.js';
  import { cfetch } from '$lib/utils.js';

  /**
   * The check settings as rows. Editing turns the rows into compact inputs in
   * place; everything else is changed on the edit page.
   * @typedef {Object} Props
   * @property {import('$lib/store/services').Service} service - Replaced with the saved one after an edit.
   * @property {boolean} [canEdit]
   */

  /** @type {Props} */
  let { service = $bindable(), canEdit = false } = $props();

  /**
   * The form while editing, else null. It holds the whole saved config, so
   * the fields without a row, secrets included, are saved back unchanged.
   * @type {import('$lib/components/service-form/service-form.js').ServiceModel | null}
   */
  let model = $state(null);
  let saving = $state(false);

  /**
   * A row: its label, its value as shown, and the model field that edits it
   * (none for rows that can only change on the edit page).
   * @typedef {[label: string, value: string, field?: string]} Row
   */

  /**
   * The HTTP status codes a check accepts. A single digit stands for a class.
   * @param {number | null | undefined} code
   */
  function acceptedStatus(code) {
    if (code == null) return '200–299';
    if (code < 10) return `${code}00–${code}99`;
    return `${code}`;
  }

  /**
   * The simple settings of each check type. Secrets such as headers, push
   * tokens and database passwords are left out.
   * @param {any} config
   * @returns {Row[]}
   */
  function typeRows(config) {
    /** @type {[string, string | null | undefined, string?][]} */
    let rows;
    switch (config?.type) {
      case 'http':
        rows = [
          ['Method', config.method ?? 'GET', 'method'],
          ['Accepted status', acceptedStatus(config.expected_code), 'expected_code'],
          ['Keyword', config.keyword, 'keyword'],
          ['JSON pointer', config.json_pointer, 'json_pointer']
        ];
        break;
      case 'tcp':
        rows = [['Port', `${config.port}`, 'port']];
        break;
      case 'dns':
        rows = [
          ['Record type', config.record_type, 'record_type'],
          ['Resolver', config.resolver, 'resolver']
        ];
        break;
      case 'tls':
        rows = [
          ['Port', `${config.port}`, 'port'],
          ['Warn before expiry', `${config.warn_days} days`, 'warn_days']
        ];
        break;
      case 'docker':
        rows = [
          ['Container', config.container, 'container'],
          ['Docker host', config.docker_host, 'docker_host']
        ];
        break;
      case 'database':
        // The engine decides whether the saved password is kept, so it only
        // changes on the edit page.
        rows = [
          ['Engine', config.engine],
          ['Database', config.database, 'db_name']
        ];
        break;
      case 'push':
        rows = [['Grace period', `${config.grace_secs}s`, 'grace_secs']];
        break;
      default:
        rows = [];
    }
    return /** @type {Row[]} */ (rows.filter(([, value]) => value));
  }

  let rows = $derived(
    /** @type {Row[]} */ ([
      ['Type', typeLabel(service.service_type)],
      ...typeRows(service.config),
      ['Interval', `Every ${service.interval}s`, 'interval'],
      ['Timeout', `${service.timeout}s`, 'timeout'],
      [
        'Retries',
        service.retry ? `${service.retry}, every ${service.retry_interval}s` : 'None',
        'retry'
      ],
      ['Invert', service.invert ? 'Yes' : 'No', 'invert']
    ])
  );

  /** Fields typed as numbers, with their unit when it isn't shown elsewhere. */
  /** @type {Record<string, string>} */
  const numbers = {
    expected_code: '',
    port: '',
    warn_days: 'd',
    grace_secs: 's',
    interval: 's',
    timeout: 's'
  };

  /**
   * A field of the model, by name, for the rows that edit it.
   * @param {string} field
   */
  const get = (field) => /** @type {Record<string, any> | null} */ (model)?.[field];

  /**
   * @param {string} field
   * @param {unknown} value
   */
  function set(field, value) {
    if (model) /** @type {Record<string, any>} */ (model)[field] = value;
  }

  /** @param {SubmitEvent} event */
  async function save(event) {
    event.preventDefault();
    if (!model) return;
    const invalid = validate(model);
    if (invalid) {
      toast.error(invalid);
      return;
    }

    let payload;
    try {
      payload = updatePayload(model);
    } catch (e) {
      toast.error(`${e instanceof Error ? e.message : e}`);
      return;
    }

    saving = true;
    try {
      const res = await cfetch(`/services/${service.id}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        credentials: 'same-origin',
        body: JSON.stringify(payload)
      });
      if (!res.ok) {
        const body = await res.json().catch(() => null);
        toast.error(`Error: ${body?.message ?? body?.error ?? res.statusText}`);
        return;
      }
      // Show the service as the server saved it.
      const reload = await cfetch(`/services/${service.id}`);
      if (reload.ok) service = (await reload.json()).service;
      model = null;
      toast.success('Check settings saved');
    } catch (e) {
      toast.error(`Error: ${e instanceof Error ? e.message : e}`);
    } finally {
      saving = false;
    }
  }
</script>

{#snippet number(
  /** @type {string} */ field,
  /** @type {string} */ label,
  /** @type {string} */ unit,
  /** @type {string} */ width
)}
  <div class="relative {width}">
    <Input
      type="number"
      min="0"
      value={get(field)}
      oninput={(/** @type {Event & { currentTarget: HTMLInputElement }} */ e) =>
        set(field, e.currentTarget.value)}
      aria-label={label}
      class="h-7 px-2 font-mono text-[13px] {unit ? 'pr-6' : ''}"
      required={field !== 'expected_code'}
    />
    {#if unit}
      <span
        class="pointer-events-none absolute inset-y-0 right-2 flex items-center text-[13px] text-muted-foreground"
      >
        {unit}
      </span>
    {/if}
  </div>
{/snippet}

<Card.Root class="gap-0 py-0">
  <div class="flex items-center justify-between gap-2 border-b px-4 py-3 sm:px-5">
    <h2 class="font-semibold">Check settings</h2>
    {#if canEdit && !model}
      <Button
        variant="link"
        size="sm"
        class="h-auto p-0"
        onclick={() => (model = fromService(service))}
      >
        Edit
      </Button>
    {/if}
  </div>
  <form onsubmit={save}>
    <dl class="divide-y text-sm">
      {#each rows as [label, value, field] (label)}
        <!-- Inputs are 28px tall, so editing rows trade padding to keep their height. -->
        <div
          class="flex items-center justify-between gap-4 px-4 sm:px-5 {model && field
            ? 'py-1.5'
            : 'py-2.5'}"
        >
          <dt class="shrink-0 text-muted-foreground">{label}</dt>
          {#if model && field}
            <dd class="flex min-w-0 items-center justify-end gap-2">
              {#if field === 'method' || field === 'record_type'}
                <Select.Root
                  type="single"
                  value={get(field)}
                  onValueChange={(/** @type {string} */ v) => set(field, v)}
                >
                  <Select.Trigger class="h-7 w-26 px-2 font-mono text-[13px]" aria-label={label}>
                    {get(field)}
                  </Select.Trigger>
                  <Select.Content>
                    {#each field === 'method' ? httpMethods : recordTypes as option (option)}
                      <Select.Item value={option} label={option}>{option}</Select.Item>
                    {/each}
                  </Select.Content>
                </Select.Root>
              {:else if field === 'retry'}
                {@render number('retry', 'Retries', '', 'w-11')}
                <span class="text-[13px] text-muted-foreground">every</span>
                {@render number('retry_interval', 'Retry interval', 's', 'w-16')}
              {:else if field === 'invert'}
                <Switch bind:checked={model.invert} aria-label="Invert" />
              {:else if field in numbers}
                {@render number(field, label, numbers[field], 'w-26')}
              {:else}
                <Input
                  value={get(field)}
                  oninput={(/** @type {Event & { currentTarget: HTMLInputElement }} */ e) =>
                    set(field, e.currentTarget.value)}
                  aria-label={label}
                  class="h-7 w-40 px-2 font-mono text-[13px]"
                  required={field === 'container'}
                />
              {/if}
            </dd>
          {:else}
            <dd class="truncate font-mono">{value}</dd>
          {/if}
        </div>
      {/each}
    </dl>
    {#if model}
      <div class="flex justify-end gap-2 border-t bg-surface px-4 py-3">
        <Button variant="outline" onclick={() => (model = null)}>Cancel</Button>
        <Button type="submit" disabled={saving}>Save changes</Button>
      </div>
    {/if}
  </form>
</Card.Root>
