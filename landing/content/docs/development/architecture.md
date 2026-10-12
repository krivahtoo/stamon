+++
title = "Architecture"
description = "How Stamon's crates, scheduler and frontend fit together."
weight = 2
+++

Stamon is a Rust workspace and a Svelte frontend, built into one Docker image.

## Crates

| Crate | Does |
|---|---|
| `crates/server` | The HTTP API and WebSocket (axum), SQLite storage (sqlx) and background jobs (apalis) |
| `crates/checks` | Check types: the `CheckConfig` enum and one module per type |
| `crates/alerts` | Notification channels: the `ChannelConfig` enum and one module per channel |
| `crates/ssl-exp` | Reads certificate expiry dates for TLS checks |

### Server modules

| Module | Does |
|---|---|
| `routes` | API handlers, nested under `/api` |
| `auth`, `extractors` | Tokens, role checks and request parsing |
| `models` | Database queries, one file per table |
| `service`, `monitors` | The scheduler and worker registration |
| `job` | The check job (`job/monitor`) and alert delivery (`job/alert.rs`) |
| `ws` | The live updates WebSocket |

Database migrations are in `crates/server/migrations/` and run on start. A
monitor's settings are stored as JSON in one column, so new check types and
channels don't need migrations.

## How a check runs

{% <devlab.steps> %}
1. A timer fires every second and finds active services whose next run is due.
2. For each, it schedules the next run and queues a check job in SQLite.
3. A worker runs the check with the service's timeout. Monitors in a
   maintenance window are skipped and logged as *Maintenance*.
4. The result is evaluated: failures count as *Pending* until the retries are
   used up, and `invert` swaps up and down.
5. The result is stored, and broadcast on `/ws` to open dashboards.
6. On a confirmed change (down, or back up), one alert job is queued per
   channel, and delivered with retries.
{% </devlab.steps> %}

## Frontend

`frontend/` is SvelteKit with the static adapter, Tailwind and
[shadcn-svelte](https://www.shadcn-svelte.com/) components. The server serves
the built files from `ASSETS_PATH` and falls back to `404.html` for client-side
routes. The frontend talks to the server through `/api` and `/ws`.
