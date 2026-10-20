+++
title = "HTTP API"
description = "Endpoints, authentication and roles."
weight = 1
+++

The dashboard uses this API, and you can script it too. All endpoints are
under `/api` and take and return JSON.

{% <devlab.callout type="info" title="Not yet stable"> %}
The API may change between 0.x releases. Check the [changelog](@/changelog.md)
when upgrading.
{% </devlab.callout> %}

## Authentication {#authentication}

Sign in to get a token:

```sh
curl -s -H 'Content-Type: application/json' \
  -d '{"username":"admin","password":"your-password"}' \
  https://stamon.example.com/api/login
# {"token":"eyJ..."}
```

Send it as `Authorization: Bearer <token>`. Signing in also sets a `token`
cookie, which the dashboard uses. Tokens last 60 minutes; sign in again for a
new one.

Each request is checked against the user's current role, so role changes and
deactivation apply right away.

| Status | Meaning |
|---|---|
| `401` | Missing or invalid token |
| `403` | The user's role is too low, or the account is deactivated |

## Endpoints {#endpoints}

The **Role** column is the minimum role needed.

### Account

| Method | Path | Role | Notes |
|---|---|---|---|
| `GET` | `/api/check` | Public | `200` once an account exists, `404` before |
| `POST` | `/api/register` | Public | `{username, password, timezone?}`; only works while there are no users, and creates the admin |
| `POST` | `/api/login` | Public | `{username, password}` → `{token}` |
| `GET` | `/api/logout` | Public | Clears the cookie |
| `GET` | `/api/user` | Viewer | The signed-in user |
| `GET` | `/api/users` | Viewer | All users |
| `POST` | `/api/users` | Admin | `{username, password, role?, timezone?}`; role defaults to `viewer` |
| `PUT` | `/api/users/{id}` | Admin | `{role?, active?}`; not for your own account |

### Services

| Method | Path | Role | Notes |
|---|---|---|---|
| `GET` | `/api/services` | Viewer | All services, with secrets hidden |
| `POST` | `/api/services` | Editor | See [Service body](#service-body) |
| `GET` | `/api/services/{id}` | Viewer | One service |
| `PUT` | `/api/services/{id}` | Editor | Any service fields to change |
| `DELETE` | `/api/services/{id}` | Editor | |
| `GET` | `/api/services/{id}/logs?limit=100` | Viewer | Recent check results |
| `GET` | `/api/services/{id}/stats` | Viewer | Uptime and latency over 24 hours, 7 days and 30 days |
| `GET` | `/api/services/{id}/channels` | Viewer | `{channel_ids}` linked to the service |
| `PUT` | `/api/services/{id}/channels` | Editor | `{channel_ids}` |
| `GET` | `/api/stats` | Viewer | Counts of services up, down and failed |
| `GET` | `/api/logs?limit=100` | Viewer | Recent check results for all services |
| `GET` | `/api/logs/incidents?limit=20` | Viewer | Recent incidents |

### Alerts, maintenance and status pages

| Method | Path | Role | Notes |
|---|---|---|---|
| `GET` | `/api/channels` | Viewer | Channels, without their settings |
| `POST` | `/api/channels` | Admin | `{name, active?, apply_to_all?, config}` |
| `GET` | `/api/channels/{id}` | Admin | Includes settings and credentials |
| `PUT` | `/api/channels/{id}` | Admin | Any channel fields to change |
| `DELETE` | `/api/channels/{id}` | Admin | |
| `POST` | `/api/channels/{id}/test` | Admin | Sends a test alert; `502` with the error if it fails |
| `GET` | `/api/notifications?limit=50` | Viewer | Alert history |
| `GET` | `/api/maintenance` | Viewer | Maintenance windows |
| `POST` | `/api/maintenance` | Editor | `{title, description?, starts_at, ends_at, apply_to_all?, service_ids?}`; times in RFC 3339 |
| `PUT`, `DELETE` | `/api/maintenance/{id}` | Editor | |
| `GET` | `/api/status-pages` | Viewer | Status pages |
| `POST` | `/api/status-pages` | Editor | `{slug, title, description?, published?, service_ids?}`; slugs use lowercase letters, digits and dashes |
| `PUT`, `DELETE` | `/api/status-pages/{id}` | Editor | |
| `GET` | `/api/status/{slug}` | Public | A published status page's data |
| `GET`, `POST` | `/api/push/{token}` | Public | [Push heartbeat](@/docs/monitors/push.md) |

## Service body {#service-body}

```json
{
  "name": "Website",
  "interval": 60,
  "timeout": 10,
  "retry": 2,
  "retry_interval": 20,
  "invert": false,
  "active": true,
  "tags": ["production"],
  "channel_ids": [1],
  "config": { "type": "http", "url": "https://example.com" }
}
```

`config.type` selects the check type, and the other `config` fields are that
type's settings. Times are in seconds. `timeout` defaults to 10, `active` to
`true` and `invert` to `false`.

## Statuses {#statuses}

Check results report status as a number:

| Value | Status |
|---|---|
| `0` | Pending (retrying) |
| `1` | Up |
| `2` | Down |
| `3` | Failed (the check itself errored) |
| `4` | Maintenance |

## Live updates {#live-updates}

`/ws` (outside `/api`) is a WebSocket that sends every new check result and
dashboard notification as JSON. It needs at least the viewer role: send the
token as the `token` cookie, which browsers do on their own, or as
`Authorization: Bearer <token>`. Without a valid token the upgrade fails with
`401`. A browser connection whose `Origin` doesn't match the `Host` (or
`X-Forwarded-Host`) header fails with `403`. The server closes the socket when the token expires; connect again with
a new one.

```json
{"type":"log","value":{"service_id":1,"status":1,"message":null,"time":"…","duration":42}}
{"type":"notification","value":{"title":"Service Down","message":"…","level":"error"}}
```
