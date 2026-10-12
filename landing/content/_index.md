+++
title = "Stamon"
sort_by = "weight"
+++

Stamon is a lightweight status monitoring software designed to provide real-time insights
into the health and performance of your services and resources.
Written in Rust for backend performance and Svelte for a minimal responsive frontend,
**Stamon** ensures efficient monitoring with a minimal resource footprint.

# Features

- **Many kinds of checks**: HTTP(S) with status, keyword and JSON assertions; ping; TCP ports; DNS records; TLS certificate expiry; Docker containers; PostgreSQL, MySQL/MariaDB and Redis; and push heartbeats for cron jobs and workers.
- **Fewer false alarms**: retries before a service is marked down, and an invert option for checks that should fail.
- **Alerts**: webhook, Slack, Discord, Telegram, ntfy and email, sent when a service goes down and when it recovers, with a delivery history.
- **Maintenance windows**: planned downtime isn't checked, doesn't alert and doesn't count against uptime.
- **Public status pages**: share a page showing the status and 30-day uptime of the services you choose.
- **Uptime and latency stats**, **tags** for organising services, and **roles** (viewer, editor, admin) for teams.
- **Real-time** updates in the dashboard, and a **lightweight** single binary backed by SQLite.

# Installation

Please see the [installation page](/installation).

# Usage

See the [usage guide](/usage) for how each feature works.

# Contributing

Thank you very much for considering contributing to this project!

We appreciate any form of contribution:

- New issues (feature requests, bug reports, questions, ideas, ...)
- Pull requests (documentation improvements, code improvements, new features, ...)