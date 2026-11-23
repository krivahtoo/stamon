<script>
  import Copy from '@lucide/svelte/icons/copy';
  import { toast } from 'svelte-sonner';
  import { Button } from '$lib/components/ui/button/index.js';
  import { Input } from '$lib/components/ui/input/index.js';
  import * as Select from '$lib/components/ui/select/index.js';
  import { Switch } from '$lib/components/ui/switch/index.js';
  import { Textarea } from '$lib/components/ui/textarea/index.js';
  import { cn, pushUrl } from '$lib/utils.js';
  import FormField from './form-field.svelte';
  import {
    enginePorts,
    httpMethods,
    keepsPassword,
    needsPassword,
    recordTypes
  } from './service-form.js';

  /**
   * The name and the fields of the model's check type.
   * @typedef {Object} Props
   * @property {import('./service-form.js').ServiceModel} model
   * @property {boolean} [stacked] - One column, for narrow places like the detail side column.
   */

  /** @type {Props} */
  let { model = $bindable(), stacked = false } = $props();

  let pair = $derived(cn('grid grid-cols-1 gap-4', !stacked && 'sm:grid-cols-2'));
  let type = $derived(model.service_type);
  let url = $derived(pushUrl(model.push_token));

  async function copyPushUrl() {
    try {
      await navigator.clipboard.writeText(url);
      toast.success('Push URL copied');
    } catch {
      toast.error('Could not copy the push URL');
    }
  }
</script>

<div class="flex flex-col gap-4">
  <FormField label="Name" id="name">
    <Input id="name" bind:value={model.name} placeholder="e.g. api-gateway" required />
  </FormField>

  {#if type === 'http'}
    <FormField label="URL" id="url">
      <Input
        id="url"
        bind:value={model.url}
        placeholder="https://example.com/health"
        type="url"
        class="font-mono"
        required
      />
    </FormField>
    <div class={pair}>
      <FormField label="Method" id="method">
        <Select.Root type="single" bind:value={model.method} name="method">
          <Select.Trigger class="w-full" id="method">{model.method}</Select.Trigger>
          <Select.Content>
            {#each httpMethods as method (method)}
              <Select.Item value={method} label={method}>{method}</Select.Item>
            {/each}
          </Select.Content>
        </Select.Root>
      </FormField>
      <FormField
        label="Expected code"
        id="expected_code"
        hint="Status code, or 1–5 for a whole class (2 = any 2xx)"
      >
        <Input
          id="expected_code"
          bind:value={model.expected_code}
          placeholder="2"
          type="number"
          min="0"
          max="599"
        />
      </FormField>
    </div>
    <FormField label="Keyword" id="keyword" optional>
      <Input id="keyword" bind:value={model.keyword} placeholder="Text the response must contain" />
    </FormField>
    <div class={pair}>
      <FormField label="JSON value" id="json_pointer" optional>
        <Input
          id="json_pointer"
          bind:value={model.json_pointer}
          placeholder="Pointer, e.g. /status"
          class="font-mono"
        />
      </FormField>
      <FormField label="Must equal" id="json_expected" optional>
        <Input
          id="json_expected"
          bind:value={model.json_expected}
          placeholder="e.g. ok"
          class="font-mono"
        />
      </FormField>
    </div>
    <FormField label="Headers" id="headers" optional hint="One header per line.">
      <Textarea
        id="headers"
        bind:value={model.headers}
        placeholder="Authorization: Bearer <token>"
        class="font-mono"
      />
    </FormField>
    <FormField label="Body" id="body" optional hint="Sent as the request body.">
      <Textarea
        id="body"
        bind:value={model.body}
        placeholder={'{"ping": true}'}
        class="font-mono"
      />
    </FormField>
    <FormField
      label="Expected payload"
      id="expected_payload"
      optional
      hint="JSON the whole response must equal."
    >
      <Textarea
        id="expected_payload"
        bind:value={model.expected_payload}
        placeholder={'{"status": "ok"}'}
        class="font-mono"
      />
    </FormField>
  {:else if type === 'ping' || type === 'tcp' || type === 'dns' || type === 'tls'}
    <div class={type === 'tcp' || type === 'tls' ? pair : 'flex flex-col'}>
      <FormField label="Host" id="host">
        <Input
          id="host"
          bind:value={model.host}
          placeholder={type === 'dns'
            ? 'Name to look up, e.g. example.com'
            : 'IP address or hostname'}
          class="font-mono"
          required
        />
      </FormField>
      {#if type === 'tcp' || type === 'tls'}
        <FormField label="Port" id="port" optional={type === 'tls'}>
          <Input
            id="port"
            bind:value={model.port}
            placeholder={type === 'tls' ? '443' : 'e.g. 5432'}
            type="number"
            min="1"
            max="65535"
            required={type === 'tcp'}
          />
        </FormField>
      {/if}
    </div>
    {#if type === 'tls'}
      <FormField label="Warn days" id="warn_days" hint="Report down this many days before expiry.">
        <Input id="warn_days" bind:value={model.warn_days} type="number" min="0" />
      </FormField>
    {/if}
    {#if type === 'dns'}
      <div class={pair}>
        <FormField label="Record type" id="record_type">
          <Select.Root type="single" bind:value={model.record_type} name="record_type">
            <Select.Trigger class="w-full" id="record_type">{model.record_type}</Select.Trigger>
            <Select.Content>
              {#each recordTypes as recordType (recordType)}
                <Select.Item value={recordType} label={recordType}>{recordType}</Select.Item>
              {/each}
            </Select.Content>
          </Select.Root>
        </FormField>
        <FormField label="Resolver" id="resolver" optional hint="Default: the system resolver.">
          <Input
            id="resolver"
            bind:value={model.resolver}
            placeholder="e.g. 1.1.1.1"
            class="font-mono"
          />
        </FormField>
      </div>
      <FormField
        label="Expected values"
        id="expected_records"
        optional
        hint="Values that must be answered, one per line."
      >
        <Textarea
          id="expected_records"
          bind:value={model.expected_records}
          placeholder="93.184.215.14"
          class="font-mono"
        />
      </FormField>
    {/if}
  {:else if type === 'docker'}
    <FormField label="Container" id="container">
      <Input
        id="container"
        bind:value={model.container}
        placeholder="Container name or id"
        class="font-mono"
        required
      />
    </FormField>
    <FormField
      label="Docker host"
      id="docker_host"
      optional
      hint="Or tcp://host:2375. When Stamon runs in Docker, mount the socket into its container."
    >
      <Input
        id="docker_host"
        bind:value={model.docker_host}
        placeholder="unix:///var/run/docker.sock"
        class="font-mono"
      />
    </FormField>
  {:else if type === 'database'}
    <div class={pair}>
      <FormField label="Host" id="db_host">
        <Input
          id="db_host"
          bind:value={model.host}
          placeholder="db.internal"
          class="font-mono"
          required
        />
      </FormField>
      <FormField label="Port" id="db_port" optional>
        <Input
          id="db_port"
          bind:value={model.db_port}
          type="number"
          min="1"
          max="65535"
          placeholder={`${enginePorts[model.db_engine] ?? ''}`}
        />
      </FormField>
    </div>
    <div class={pair}>
      <FormField label="Username" id="db_username" optional>
        <Input id="db_username" bind:value={model.db_username} autocomplete="off" />
      </FormField>
      <FormField
        label="Password"
        id="db_password"
        optional={!needsPassword(model)}
        hint={needsPassword(model) ? 'Re-enter the password for the new server.' : undefined}
      >
        <Input
          id="db_password"
          bind:value={model.db_password}
          type="password"
          placeholder={keepsPassword(model) ? 'Unchanged' : ''}
          autocomplete="new-password"
          required={needsPassword(model)}
        />
      </FormField>
    </div>
    <FormField label="Database" id="db_name" optional>
      <Input
        id="db_name"
        bind:value={model.db_name}
        placeholder={model.db_engine === 'redis' ? 'Database number, e.g. 0' : ''}
      />
    </FormField>
    {#if model.db_engine !== 'redis'}
      <div class="flex items-center justify-between gap-4 rounded-lg border px-3.5 py-3">
        <label for="db_tls" class="text-sm font-semibold">Require TLS</label>
        <Switch id="db_tls" bind:checked={model.db_tls} />
      </div>
    {/if}
    <p class="text-xs text-muted-foreground">
      Use a read-only account: the check only connects and runs a trivial query.
    </p>
  {:else if type === 'push'}
    <FormField label="Push URL" id="push_url">
      <div class="flex gap-2">
        <Input id="push_url" readonly value={url} class="font-mono text-xs" />
        <Button variant="outline" onclick={copyPushUrl}>
          <Copy />
          Copy
        </Button>
      </div>
      <p class="text-xs text-muted-foreground">
        Have your job call this URL at least once per interval. Add
        <code class="font-mono">?status=down&amp;msg=...</code> to report a failure.
      </p>
    </FormField>
    <FormField
      label="Grace period"
      id="grace_secs"
      hint="Extra seconds before a heartbeat counts as missed."
    >
      <Input id="grace_secs" bind:value={model.grace_secs} type="number" min="0" />
    </FormField>
  {/if}
</div>
