import { describe, expect, it } from 'vitest';
import {
  REDACTED,
  buildConfig,
  createPayload,
  emptyService,
  fromService,
  kindOf,
  needsPassword,
  setKind,
  updatePayload,
  validate
} from './service-form.js';

const defaults = { retry: 1, retry_interval: 30, interval: 60, timeout: 20 };

/**
 * @param {Record<string, any>} config
 * @returns {import('$lib/store/services').Service}
 */
const saved = (config) => ({
  id: 7,
  active: true,
  name: 'api-gateway',
  service_type: config.type,
  target: null,
  config,
  tags: ['prod', 'api'],
  last_status: 1,
  interval: 120,
  timeout: 10,
  retry: 2,
  retry_interval: 20,
  invert: true
});

describe('emptyService', () => {
  it('starts as an HTTP check with the defaults', () => {
    const model = emptyService(defaults);
    expect(model.service_type).toBe('http');
    expect(model.interval).toBe(60);
    expect(model.saved).toBeNull();
  });
});

describe('setKind', () => {
  it('picks the engine from a database card', () => {
    const model = emptyService(defaults);
    setKind(model, 'redis');
    expect(model.service_type).toBe('database');
    expect(model.db_engine).toBe('redis');
    expect(kindOf(model)).toBe('redis');
  });
});

describe('createPayload', () => {
  it('sends tags and channels with the settings', () => {
    const model = emptyService(defaults);
    Object.assign(model, {
      name: 'web',
      url: 'https://example.com',
      headers: 'Accept: application/json',
      tags: ['prod'],
      channel_ids: [3]
    });
    expect(createPayload(model)).toMatchObject({
      name: 'web',
      interval: 60,
      tags: ['prod'],
      channel_ids: [3],
      config: { type: 'http', headers: { Accept: 'application/json' }, body: null }
    });
  });
});

describe('fromService', () => {
  it('round-trips an HTTP check', () => {
    const config = {
      type: 'http',
      url: 'https://api.acme.dev/health',
      method: 'POST',
      headers: { Authorization: REDACTED },
      body: '{"ping": true}',
      expected_code: null,
      expected_payload: null,
      keyword: 'ok',
      json_pointer: null,
      json_expected: null
    };
    const model = fromService(saved(config));
    expect(model.headers).toBe(`Authorization: ${REDACTED}`);
    expect(model.expected_code).toBe('');
    expect(updatePayload(model)).toEqual({
      name: 'api-gateway',
      retry: 2,
      retry_interval: 20,
      interval: 120,
      timeout: 10,
      invert: true,
      config
    });
  });

  it('round-trips a DNS check', () => {
    const config = {
      type: 'dns',
      host: 'example.com',
      record_type: 'MX',
      resolver: null,
      expected: ['mx1.example.com', 'mx2.example.com']
    };
    expect(buildConfig(fromService(saved(config)))).toEqual(config);
  });
});

describe('saved secrets', () => {
  const database = {
    type: 'database',
    engine: 'postgres',
    host: 'db.internal',
    port: null,
    username: 'stamon',
    password: REDACTED,
    database: null,
    tls: false
  };

  it('keeps the password for the same server', () => {
    const model = fromService(saved(database));
    // The default port is the same server as no port.
    model.db_port = '5432';
    expect(needsPassword(model)).toBe(false);
    expect(buildConfig(model)).toMatchObject({ password: REDACTED });
  });

  it('asks for the password again on a new server', () => {
    const model = fromService(saved(database));
    model.host = 'db2.internal';
    expect(needsPassword(model)).toBe(true);
    expect(validate(model)).toMatch(/password/);
    expect(buildConfig(model)).toMatchObject({ password: null });

    model.db_password = 'hunter2';
    expect(validate(model)).toBeNull();
    expect(buildConfig(model)).toMatchObject({ password: 'hunter2' });
  });

  it('sends no password when none was saved', () => {
    const model = fromService(saved({ ...database, password: null }));
    model.host = 'db2.internal';
    expect(needsPassword(model)).toBe(false);
    expect(buildConfig(model)).toMatchObject({ password: null });
  });

  it('asks for hidden headers again on a new origin', () => {
    const model = fromService(
      saved({ type: 'http', url: 'https://api.acme.dev/health', headers: { Auth: REDACTED } })
    );
    model.url = 'https://api.acme.dev/ready';
    expect(validate(model)).toBeNull();
    model.url = 'https://evil.example/health';
    expect(validate(model)).toMatch(/header/);
  });
});

describe('validate', () => {
  it('wants the timeout under the interval, except for push', () => {
    const model = emptyService({ ...defaults, timeout: 60 });
    expect(validate(model)).toMatch(/Timeout/);
    setKind(model, 'push');
    expect(validate(model)).toBeNull();
  });
});
