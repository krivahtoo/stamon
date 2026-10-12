+++
title = "Stamon v0.3.0: new checks, alerts and status pages"
description = "Stamon 0.3.0 adds TCP, DNS, TLS, push, Docker and database checks, notification channels, maintenance windows, public status pages and roles."
date = 2026-10-12

[taxonomies]
tags = ["release"]
authors = ["krivahtoo"]

[extra]
release = "v0.3.0"
+++

Stamon 0.3.0 is the biggest release so far. It grows Stamon from an HTTP
checker into a monitoring tool that can watch most of a small team's stack,
tell you when something breaks, and show your users what is going on.

<!-- more -->

## More kinds of checks

Checks now live in their own crate with a per-type JSON configuration, which
made room for several new types:

- **TCP ports**, **DNS records** and **TLS certificate expiry**.
- **Docker containers**, through the Docker socket or a TCP address.
- **PostgreSQL, MySQL/MariaDB and Redis**, with a trivial query against the server.
- **Push monitors** for cron jobs, backups and workers: the job calls a secret
  URL, and the monitor goes down if no heartbeat arrives in time.

Every monitor also gets retries, a retry interval and an invert option, so a
single slow response no longer raises an alarm.

## Alerts

Admins can add **notification channels**: webhook, Slack, Discord, Telegram,
ntfy and email. Alerts are sent when a service is confirmed down and when it
recovers. Failed deliveries are retried, and a history shows every alert and
whether it arrived.

## Uptime, maintenance and status pages

- **Uptime and latency stats**, and **tags** for organising services.
- **Maintenance windows**: covered services aren't checked, don't alert and
  don't count against uptime while the window is active.
- **Public status pages** show the status and 30-day uptime of the services you
  choose, without exposing URLs or settings.

## Roles

Users now have a **viewer**, **editor** or **admin** role, and the server
enforces it. Push tokens and other secrets are hidden from viewers.

## Upgrade

With Docker, pull the new image and recreate the container:

```sh
docker pull k4htoo/stamon
docker compose up -d
```

Keep the `stamon-data` volume so the database survives the upgrade, and set a
strong `JWT_SECRET`. See the [installation guide](@/docs/installation.md) for a
full setup, the [usage guide](@/docs/usage.md) for how each feature works, and
the [changelog](@/docs/changelog.md) for every change in this release.
