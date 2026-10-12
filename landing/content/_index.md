+++
title = "Monitor your services"
description = "A lightweight self-hosted status monitoring tool, written in Rust."
sort_by = "weight"

[[extra.devlab.home.blocks]]
type = "hero"
eyebrow = "Self-hosted · Written in Rust"
image = "/screenshot.png"
image_alt = "The Stamon dashboard listing monitored services with their status and response times."
actions = [
  { label = "Install", path = "@/docs/installation.md" },
  { label = "View on GitHub", path = "https://github.com/krivahtoo/stamon" },
]

[[extra.devlab.home.blocks]]
type = "features"
items = [
  { kicker = "Checks", title = "Many kinds of checks", description = "HTTP(S) with status, keyword and JSON assertions; ping; TCP ports; DNS records; TLS certificate expiry; Docker containers; PostgreSQL, MySQL/MariaDB and Redis; and push heartbeats for cron jobs and workers.", path = "@/docs/usage.md#monitors" },
  { kicker = "Alerts", title = "Alerts where you work", description = "Webhook, Slack, Discord, Telegram, ntfy and email, sent when a service goes down and when it recovers, with a delivery history.", path = "@/docs/usage.md#alerts" },
  { kicker = "Status pages", title = "Public status pages", description = "Share a page showing the status and 30-day uptime of the services you choose.", path = "@/docs/usage.md#status-pages" },
]

[[extra.devlab.home.blocks]]
type = "learn"
eyebrow = "Features"
title = "Built for small teams"
description = "Everything you need to know when a service goes down, in a single lightweight binary backed by SQLite."
items = [
  { kicker = "Reliability", title = "Fewer false alarms", description = "Retries before a service is marked down, and an invert option for checks that should fail.", path = "@/docs/usage.md#monitors" },
  { kicker = "Planning", title = "Maintenance windows", description = "Planned downtime isn't checked, doesn't alert and doesn't count against uptime.", path = "@/docs/usage.md#maintenance-windows" },
  { kicker = "Insight", title = "Uptime and latency stats", description = "Track uptime and response times, and organise services with tags.", path = "@/docs/usage.md#data-and-retention" },
  { kicker = "Teams", title = "Roles", description = "Viewer, editor and admin roles, with real-time updates in the dashboard.", path = "@/docs/usage.md#users-and-roles" },
]

[[extra.devlab.home.blocks]]
type = "content"

[[extra.devlab.home.blocks]]
type = "cta"
title = "Start monitoring in a minute"
description = "Run Stamon with Docker, register the first account and add your first monitor."
actions = [
  { label = "Install Stamon", path = "@/docs/installation.md" },
  { label = "Read the usage guide", path = "@/docs/usage.md", style = "secondary" },
]
+++

## Overview

Stamon is a lightweight status monitoring software designed to provide real-time insights
into the health and performance of your services and resources.
Written in Rust for backend performance and Svelte for a minimal responsive frontend,
**Stamon** ensures efficient monitoring with a minimal resource footprint.

## Contributing

Thank you very much for considering contributing to this project!

We appreciate any form of contribution:

- New issues (feature requests, bug reports, questions, ideas, ...)
- Pull requests (documentation improvements, code improvements, new features, ...)
