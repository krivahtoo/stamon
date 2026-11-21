<script>
  import * as Card from '$lib/components/ui/card/index.js';
  import { typeLabel } from '$lib/data/checks.js';

  /**
   * @typedef {Object} Props
   * @property {import('$lib/store/services').Service} service
   */

  /** @type {Props} */
  let { service } = $props();

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
   * @returns {[string, string][]}
   */
  function typeRows(config) {
    /** @type {[string, string | null | undefined][]} */
    let rows;
    switch (config?.type) {
      case 'http':
        rows = [
          ['Method', config.method ?? 'GET'],
          ['Accepted status', acceptedStatus(config.expected_code)],
          ['Keyword', config.keyword],
          ['JSON pointer', config.json_pointer]
        ];
        break;
      case 'tcp':
        rows = [['Port', `${config.port}`]];
        break;
      case 'dns':
        rows = [
          ['Record type', config.record_type],
          ['Resolver', config.resolver]
        ];
        break;
      case 'tls':
        rows = [
          ['Port', `${config.port}`],
          ['Warn before expiry', `${config.warn_days} days`]
        ];
        break;
      case 'docker':
        rows = [
          ['Container', config.container],
          ['Docker host', config.docker_host]
        ];
        break;
      case 'database':
        rows = [
          ['Engine', config.engine],
          ['Database', config.database]
        ];
        break;
      case 'push':
        rows = [['Grace period', `${config.grace_secs}s`]];
        break;
      default:
        rows = [];
    }
    return /** @type {[string, string][]} */ (rows.filter(([, value]) => value));
  }

  let rows = $derived(
    /** @type {[string, string][]} */ ([
      ['Type', typeLabel(service.service_type)],
      ...typeRows(service.config),
      ['Interval', `Every ${service.interval}s`],
      ['Timeout', `${service.timeout}s`],
      ['Retries', service.retry ? `${service.retry}, every ${service.retry_interval}s` : 'None'],
      ['Invert', service.invert ? 'Yes' : 'No']
    ])
  );
</script>

<Card.Root class="gap-0 py-0">
  <h2 class="border-b px-4 py-3 font-semibold sm:px-5">Check settings</h2>
  <dl class="divide-y text-sm">
    {#each rows as [label, value] (label)}
      <div class="flex items-center justify-between gap-4 px-4 py-2.5 sm:px-5">
        <dt class="text-muted-foreground">{label}</dt>
        <dd class="truncate font-mono">{value}</dd>
      </div>
    {/each}
  </dl>
</Card.Root>
