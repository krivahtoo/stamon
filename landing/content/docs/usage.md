+++
title = "Usage"
description = "Using stamon"
weight = 2
aliases = ["/usage/"]
+++

This guide covers what **Stamon** can monitor and how its alerts, maintenance windows, status pages and roles work.

---

## Monitors {#monitors}

Every monitor has these settings:

- **Interval**: how often it's checked, in seconds.
- **Timeout**: how long a check may take before it counts as down.
- **Retries** and **retry interval**: how many failed checks in a row are retried, and how far apart, before the service is marked down. While retrying, the service shows as *Pending* and no alert is sent.
- **Invert**: treat success as down and failure as up, for things that should *not* be reachable.
- **Tags**: labels for grouping and finding services.
- **Alert channels**: where alerts go (see [Alerts](#alerts)).

### Check types

| Type | Up when |
|---|---|
| **HTTP(S)** | The response status matches and every body assertion passes |
| **Ping** | The host answers an ICMP echo |
| **TCP Port** | A TCP connection to the port succeeds |
| **DNS** | The name resolves, and the answers include every expected value |
| **TLS Certificate** | The certificate is valid for longer than the warning period |
| **Docker Container** | The container is running and not reported unhealthy |
| **Database** | PostgreSQL, MySQL/MariaDB or Redis accepts a connection and answers a trivial query |
| **Push** | The service sent a heartbeat within the interval (see [Push monitors](#push-monitors)) |

#### HTTP(S)

- Choose the **method**, and optionally **headers** and a **body**. A JSON body is sent as `application/json` unless you set `Content-Type` yourself.
- **Expected code**: an exact status like `204`, or a single digit for a whole class (`2` accepts any 2xx). Without one, any 2xx is up.
- **Expected payload**: JSON the whole response must equal.
- **Keyword**: text the response must contain.
- **JSON value**: a [JSON pointer](https://datatracker.ietf.org/doc/html/rfc6901) such as `/status` or `/checks/0/ok`, and optionally the value it must have. On its own, the value only has to exist.

#### DNS

Pick a record type (`A`, `AAAA`, `CNAME`, `MX`, `NS` or `TXT`) and, optionally, a resolver (`1.1.1.1` or `ip:port`; the system resolver otherwise). **Expected values** must all be in the answers, compared without case or a trailing dot, e.g. `93.184.215.14` or `10 mail.example.com`. Answers aren't cached between checks.

#### TLS Certificate

Reports down when the certificate expires within **warn days** (default 14) or has already expired.

#### Docker Container

Name the container (or its id). A container whose health check is still *starting* counts as up, so restarts don't cause alerts. Stamon talks to the Docker API at `unix:///var/run/docker.sock` by default, or at a `tcp://host:2375` address; TLS-protected daemons aren't supported yet.

When Stamon runs in Docker, mount the host's socket into its container:

```yml
services:
  stamon:
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock:ro
```

Access to the Docker socket is equivalent to root access on the host, so only mount it if you trust everyone who can manage monitors.

#### Database

Pick the engine and give the host, port (the engine's default otherwise), credentials and database (a number for Redis). Stamon connects and runs `SELECT 1`, or `PING` for Redis. Use a read-only account. **Require TLS** is available for PostgreSQL and MySQL; without it, they still use TLS when the server offers it. Redis checks don't support TLS yet.

## Push monitors {#push-monitors}

For cron jobs, backups and workers that can't be reached from Stamon, the job reports in instead. Each push monitor has a secret URL, shown when you create it and on its page to editors and admins:

```
https://stamon.example.com/api/push/<token>
```

Call it with `GET` or `POST` at least once per interval. If no heartbeat arrives within the interval plus the **grace period**, the monitor goes down. Optional query parameters:

| Parameter | Meaning |
|---|---|
| `status` | `up` (default) or `down` |
| `msg` | A message stored with the heartbeat |
| `ping` | A response time to record, in milliseconds |

For example, at the end of a backup script:

```sh
if ./backup.sh; then
  curl -fsS "https://stamon.example.com/api/push/<token>"
else
  curl -fsS "https://stamon.example.com/api/push/<token>?status=down&msg=backup%20failed"
fi
```

## Alerts {#alerts}

Alerts are sent when a service is confirmed **down** (after its retries) and when it comes back **up**. Retries, maintenance and errors in Stamon itself don't send alerts.

Admins add channels under **Notification Providers**:

| Channel | Settings |
|---|---|
| **Webhook** | URL and optional headers; receives a JSON POST with the service, status, message, `title` and `text` |
| **Slack** | Incoming webhook URL |
| **Discord** | Webhook URL |
| **Telegram** | Bot token (from @BotFather) and chat id |
| **ntfy** | Server (default `https://ntfy.sh`), topic and optional access token |
| **Email** | SMTP host, port, security (STARTTLS, TLS or none), optional username and password, sender and recipients |

- Pick channels for each service when creating it or on its page, or turn on **All services** for a channel to receive every alert.
- **Test** sends a sample alert right away.
- Failed deliveries are retried 3 times. The **History** list shows every alert and whether it was delivered.

## Maintenance windows {#maintenance-windows}

Schedule planned work under **Maintenance**, for chosen services or all of them. While a window is active, covered services aren't checked: they show as *Maintenance*, send no alerts and don't count against uptime. If a service is still down when the window ends, it's retried and then alerts as usual.

## Status pages {#status-pages}

Under **Status Pages**, create a page with a title, an address (slug) and the services to show, in order. Once **published**, anyone can open it without an account at:

```
https://stamon.example.com/status/<slug>/
```

Visitors see each service's name, current status, 30-day uptime and a bar per day, plus upcoming maintenance. They never see URLs, hosts or settings, and paused services are left out. Unpublished pages return *not found*.

## Users and roles {#users-and-roles}

| Role | Can |
|---|---|
| **Viewer** | See services, logs, stats, maintenance, status pages and alert history |
| **Editor** | Also create, edit, pause and delete monitors, and manage maintenance windows and status pages |
| **Admin** | Also manage users and notification channels |

The first account registered is an admin. Admins add users through the API for now:

```sh
# Create a user
curl -H "Authorization: Bearer $TOKEN" -H 'Content-Type: application/json' \
  -d '{"username":"alice","password":"a-strong-password","role":"viewer"}' \
  http://localhost:3000/api/users

# Change a user's role, or deactivate them (user id 2 here)
curl -X PUT -H "Authorization: Bearer $TOKEN" -H 'Content-Type: application/json' \
  -d '{"role":"editor","active":true}' \
  http://localhost:3000/api/users/2
```

Get `$TOKEN` by signing in, as shown in [First Run](@/docs/installation.md#first-run). Role changes and deactivation apply to changes right away; a deactivated user can still view until their sign-in expires (60 minutes). Admins can't change their own role or status.

## Data and retention {#data-and-retention}

- Check logs and alert history are kept for `LOG_RETENTION_DAYS` days (default 90; `0` keeps them forever).
- **Uptime** is the share of *up* results among up and down ones. Pending, failed and maintenance results don't count.
- Secrets in monitor settings are hidden from the API: database passwords and HTTP header values show as `********`, and push tokens do too for viewers. Channel settings are only visible to admins.
