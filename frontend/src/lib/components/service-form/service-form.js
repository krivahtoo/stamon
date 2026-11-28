/**
 * The service form's model, and conversions to and from the API. Shared by
 * the new service page and the check settings editor on the detail page.
 */

/** What the API shows instead of a secret; saving it back keeps the secret. */
export const REDACTED = '********';

/**
 * The cards of the check type grid. The database engines get a card each.
 * @type {{ value: string, type: string, engine?: string, label: string, description: string }[]}
 */
export const checkKinds = [
  {
    value: 'http',
    type: 'http',
    label: 'HTTP(s)',
    description: 'Status code, keyword or JSON value from a URL'
  },
  { value: 'ping', type: 'ping', label: 'Ping', description: 'ICMP echo to a host' },
  { value: 'tcp', type: 'tcp', label: 'TCP Port', description: 'Opens a connection to a port' },
  {
    value: 'dns',
    type: 'dns',
    label: 'DNS',
    description: 'Resolves a record and compares its values'
  },
  {
    value: 'tls',
    type: 'tls',
    label: 'TLS Certificate',
    description: 'Warns before a certificate expires'
  },
  {
    value: 'docker',
    type: 'docker',
    label: 'Docker Container',
    description: 'Checks that a container is running'
  },
  {
    value: 'postgres',
    type: 'database',
    engine: 'postgres',
    label: 'PostgreSQL',
    description: 'Connects to the database server'
  },
  {
    value: 'mysql',
    type: 'database',
    engine: 'mysql',
    label: 'MySQL / MariaDB',
    description: 'Connects to the database server'
  },
  {
    value: 'redis',
    type: 'database',
    engine: 'redis',
    label: 'Redis',
    description: 'Connects and sends PING'
  },
  {
    value: 'push',
    type: 'push',
    label: 'Push (heartbeat)',
    description: 'Your job calls Stamon on a schedule'
  }
];

/** Default ports of the database engines. */
/** @type {Record<string, number>} */
export const enginePorts = { postgres: 5432, mysql: 3306, redis: 6379 };

export const httpMethods = ['GET', 'HEAD', 'POST', 'PUT', 'PATCH', 'DELETE', 'OPTIONS'];

export const recordTypes = ['A', 'AAAA', 'CNAME', 'MX', 'NS', 'TXT'];

/** Tag limits, as the server enforces them. */
export const MAX_TAGS = 10;
export const MAX_TAG_LEN = 32;

/**
 * @typedef {Object} ServiceModel
 * @property {string} name - The monitor name.
 * @property {string} service_type - The check type.
 * @property {number | string} retry - The number of retries.
 * @property {number | string} retry_interval - The interval between retries in seconds.
 * @property {number | string} interval - The monitoring interval in seconds.
 * @property {number | string} timeout - The check timeout in seconds.
 * @property {boolean} invert - Invert the expected result.
 * @property {string} url - HTTP: the URL to request.
 * @property {string} method - HTTP: the request method.
 * @property {string} headers - HTTP: request headers, one `Name: value` per line.
 * @property {string} body - HTTP: the request body.
 * @property {number | string} expected_code - HTTP: expected status code, or 1-5 for a class.
 * @property {string} expected_payload - HTTP: JSON the response must equal.
 * @property {string} keyword - HTTP: text the response must contain.
 * @property {string} json_pointer - HTTP: JSON pointer to a value in the response.
 * @property {string} json_expected - HTTP: what the value at json_pointer must be.
 * @property {string} host - Ping, TCP, DNS, TLS, Database: the IP address or hostname.
 * @property {number | string} port - TCP, TLS: the port.
 * @property {string} record_type - DNS: the record type to look up.
 * @property {string} resolver - DNS: nameserver to ask, `ip` or `ip:port`.
 * @property {string} expected_records - DNS: values that must be answered, one per line.
 * @property {number | string} warn_days - TLS: days before expiry to report down.
 * @property {string} push_token - Push: the secret in the push URL.
 * @property {number | string} grace_secs - Push: extra seconds before a heartbeat is late.
 * @property {string} container - Docker: the container name or id.
 * @property {string} docker_host - Docker: `unix://` or `tcp://` address of the daemon.
 * @property {string} db_engine - Database: postgres, mysql or redis.
 * @property {number | string} db_port - Database: the port, empty for the engine's default.
 * @property {string} db_username - Database: the user to connect as.
 * @property {string} db_password - Database: a new password; empty keeps the saved one.
 * @property {string} db_name - Database: the database name, or number for Redis.
 * @property {boolean} db_tls - Database: require TLS.
 * @property {number[]} channel_ids - Channels that receive this service's alerts.
 * @property {string[]} tags - Labels for grouping and finding the service.
 * @property {Saved | null} saved - What the edited service was saved with; null for a new one.
 */

/**
 * The parts of a saved config that decide whether the server keeps its secrets.
 * @typedef {Object} Saved
 * @property {string} url - HTTP: hidden header values are kept for the same origin.
 * @property {boolean} hasPassword - Database: a password is saved.
 * @property {string} server - Database: engine, host and port; the password is kept for the same one.
 */

/** A random token for a push URL; works without a secure context. */
export function newPushToken() {
  const bytes = crypto.getRandomValues(new Uint8Array(16));
  return Array.from(bytes, (b) => b.toString(16).padStart(2, '0')).join('');
}

/**
 * @param {{ retry: number, retry_interval: number, interval: number, timeout: number }} defaults
 * @returns {ServiceModel}
 */
export function emptyService(defaults) {
  return {
    name: '',
    service_type: 'http',
    ...defaults,
    invert: false,
    url: '',
    method: 'GET',
    headers: '',
    body: '',
    expected_code: 2,
    expected_payload: '',
    keyword: '',
    json_pointer: '',
    json_expected: '',
    host: '',
    port: '',
    record_type: 'A',
    resolver: '',
    expected_records: '',
    warn_days: 14,
    push_token: newPushToken(),
    grace_secs: 0,
    container: '',
    docker_host: '',
    db_engine: 'postgres',
    db_port: '',
    db_username: '',
    db_password: '',
    db_name: '',
    db_tls: false,
    channel_ids: [],
    tags: [],
    saved: null
  };
}

/**
 * The form model of a saved service, for editing it.
 * @param {import('$lib/store/services').Service} service
 * @returns {ServiceModel}
 */
export function fromService(service) {
  const config = /** @type {Record<string, any>} */ (service.config);
  const { retry, retry_interval, interval, timeout } = service;
  const model = emptyService({ retry, retry_interval, interval, timeout });
  const text = (/** @type {unknown} */ value) => (value == null ? '' : `${value}`);

  Object.assign(model, {
    name: service.name,
    service_type: config.type,
    invert: service.invert,
    tags: [...(service.tags ?? [])]
  });
  switch (config.type) {
    case 'http':
      Object.assign(model, {
        url: config.url,
        method: config.method ?? 'GET',
        headers: Object.entries(config.headers ?? {})
          .map(([name, value]) => `${name}: ${value}`)
          .join('\n'),
        body: text(config.body),
        expected_code: text(config.expected_code),
        expected_payload: text(config.expected_payload),
        keyword: text(config.keyword),
        json_pointer: text(config.json_pointer),
        json_expected: text(config.json_expected)
      });
      break;
    case 'ping':
      model.host = config.host;
      break;
    case 'tcp':
    case 'tls':
      Object.assign(model, { host: config.host, port: text(config.port) });
      if (config.type === 'tls') model.warn_days = config.warn_days;
      break;
    case 'dns':
      Object.assign(model, {
        host: config.host,
        record_type: config.record_type,
        resolver: text(config.resolver),
        expected_records: (config.expected ?? []).join('\n')
      });
      break;
    case 'docker':
      Object.assign(model, { container: config.container, docker_host: text(config.docker_host) });
      break;
    case 'database':
      Object.assign(model, {
        db_engine: config.engine,
        host: config.host,
        db_port: text(config.port),
        db_username: text(config.username),
        db_name: text(config.database),
        db_tls: config.tls ?? false
      });
      break;
    case 'push':
      Object.assign(model, { push_token: config.token, grace_secs: config.grace_secs ?? 0 });
      break;
  }
  model.saved = {
    url: model.url,
    hasPassword: config.type === 'database' && config.password != null,
    server: databaseServer(model)
  };
  return model;
}

/**
 * The card a model's type is shown as.
 * @param {ServiceModel} model
 */
export const kindOf = (model) =>
  model.service_type === 'database' ? model.db_engine : model.service_type;

/**
 * Pick a card of the check type grid.
 * @param {ServiceModel} model
 * @param {string} value
 */
export function setKind(model, value) {
  const kind = checkKinds.find((k) => k.value === value);
  if (!kind) return;
  model.service_type = kind.type;
  if (kind.engine) model.db_engine = kind.engine;
}

/**
 * The label of a model's check type, for section titles.
 * @param {ServiceModel} model
 */
export const kindLabel = (model) =>
  checkKinds.find((k) => k.value === kindOf(model))?.label ?? model.service_type;

/** @param {ServiceModel} model */
const databaseServer = (model) =>
  [
    model.db_engine,
    model.host.trim(),
    model.db_port === '' ? enginePorts[model.db_engine] : Number(model.db_port)
  ].join(' ');

/**
 * Whether the saved database password is kept, which the server only does
 * for the same engine, host and port.
 * @param {ServiceModel} model
 */
export const keepsPassword = (model) =>
  !!model.saved?.hasPassword && databaseServer(model) === model.saved.server;

/**
 * Whether a new password has to be typed: one was saved, but the server changed.
 * @param {ServiceModel} model
 */
export const needsPassword = (model) =>
  !!model.saved?.hasPassword && !model.db_password && !keepsPassword(model);

/** @param {string} url */
const origin = (url) => {
  try {
    return new URL(url).origin;
  } catch {
    return null;
  }
};

/**
 * Parse `Name: value` lines into a header map.
 * @param {string} text
 * @returns {Record<string, string>}
 */
export function parseHeaders(text) {
  /** @type {Record<string, string>} */
  const headers = {};
  for (const line of text.split('\n')) {
    if (!line.trim()) continue;
    const split = line.indexOf(':');
    if (split <= 0) throw new Error(`Invalid header line: "${line}"`);
    headers[line.slice(0, split).trim()] = line.slice(split + 1).trim();
  }
  return headers;
}

/**
 * Build the type-specific check config sent to the API.
 * @param {ServiceModel} model
 */
export function buildConfig(model) {
  switch (model.service_type) {
    case 'http':
      return {
        type: 'http',
        url: model.url,
        method: model.method,
        headers: parseHeaders(model.headers),
        body: model.body || null,
        expected_code: model.expected_code === '' ? null : Number(model.expected_code),
        expected_payload: model.expected_payload || null,
        keyword: model.keyword || null,
        json_pointer: model.json_pointer || null,
        json_expected: model.json_expected || null
      };
    case 'ping':
      return { type: 'ping', host: model.host };
    case 'tcp':
      return { type: 'tcp', host: model.host, port: Number(model.port) };
    case 'dns':
      return {
        type: 'dns',
        host: model.host,
        record_type: model.record_type,
        resolver: model.resolver || null,
        expected: model.expected_records
          .split('\n')
          .map((v) => v.trim())
          .filter(Boolean)
      };
    case 'tls':
      return {
        type: 'tls',
        host: model.host,
        port: Number(model.port) || 443,
        warn_days: Number(model.warn_days)
      };
    case 'docker':
      return {
        type: 'docker',
        container: model.container,
        docker_host: model.docker_host || null
      };
    case 'database':
      return {
        type: 'database',
        engine: model.db_engine,
        host: model.host,
        port: model.db_port === '' ? null : Number(model.db_port),
        username: model.db_username || null,
        password: model.db_password || (keepsPassword(model) ? REDACTED : null),
        database: model.db_name || null,
        tls: model.db_tls
      };
    case 'push':
      return {
        type: 'push',
        token: model.push_token,
        grace_secs: Number(model.grace_secs) || 0
      };
    default:
      throw new Error('Select a check type');
  }
}

/**
 * Why the model can't be saved, or null when it can.
 * @param {ServiceModel} model
 * @returns {string | null}
 */
export function validate(model) {
  // Push monitors aren't checked, so they have no timeout.
  if (model.service_type !== 'push' && Number(model.timeout) >= Number(model.interval)) {
    return 'Timeout must be less than interval';
  }
  if (needsPassword(model)) {
    return 'Re-enter the database password for the new server';
  }
  if (
    model.saved &&
    model.service_type === 'http' &&
    model.headers.includes(REDACTED) &&
    origin(model.url) !== origin(model.saved.url)
  ) {
    return 'Re-enter the hidden header values for the new URL';
  }
  return null;
}

/**
 * The schedule and check settings, as both creating and updating take them.
 * @param {ServiceModel} model
 */
function settings(model) {
  return {
    name: model.name,
    retry: Number(model.retry),
    retry_interval: Number(model.retry_interval),
    interval: Number(model.interval),
    timeout: Number(model.timeout),
    invert: model.invert,
    config: buildConfig(model)
  };
}

/**
 * The body of `POST /services`.
 * @param {ServiceModel} model
 */
export const createPayload = (model) => ({
  ...settings(model),
  tags: model.tags,
  channel_ids: model.channel_ids
});

/**
 * The body of `PUT /services/{id}` from the check settings card, which
 * leaves tags and channels to their own editors.
 * @param {ServiceModel} model
 */
export const updatePayload = (model) => settings(model);

/**
 * The body of `PUT /services/{id}` from the edit page. Channels are saved
 * separately, with `PUT /services/{id}/channels`.
 * @param {ServiceModel} model
 */
export const editPayload = (model) => ({ ...settings(model), tags: model.tags });
