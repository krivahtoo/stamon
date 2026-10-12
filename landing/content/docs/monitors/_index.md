+++
title = "Monitors"
description = "What Stamon can check, and the settings every monitor shares."
weight = 2
sort_by = "weight"
template = "docs.html"
page_template = "doc-page.html"
aliases = ["/usage/", "/docs/usage/"]
+++

A monitor checks one service on a schedule and records whether it is up.

## Shared settings {#settings}

Every monitor has these settings:

- **Interval**: how often it's checked, in seconds.
- **Timeout**: how long a check may take before it counts as down.
- **Retries** and **retry interval**: how many failed checks in a row are retried,
  and how far apart, before the service is marked down. While retrying, the
  service shows as *Pending* and no alert is sent.
- **Invert**: treat success as down and failure as up, for things that should
  *not* be reachable.
- **Tags**: labels for grouping and finding services.
- **Alert channels**: where alerts go (see [Alerts](@/docs/features/alerts.md)).

## Check types {#check-types}

| Type | Up when |
|---|---|
| [HTTP(S)](@/docs/monitors/http.md) | The response status matches and every body assertion passes |
| [Ping](@/docs/monitors/ping.md) | The host answers an ICMP echo |
| [TCP port](@/docs/monitors/tcp.md) | A TCP connection to the port succeeds |
| [DNS](@/docs/monitors/dns.md) | The name resolves, and the answers include every expected value |
| [TLS certificate](@/docs/monitors/tls.md) | The certificate is valid for longer than the warning period |
| [Docker container](@/docs/monitors/docker.md) | The container is running and not reported unhealthy |
| [Database](@/docs/monitors/database.md) | PostgreSQL, MySQL/MariaDB or Redis accepts a connection and answers a trivial query |
| [Push](@/docs/monitors/push.md) | The service sent a heartbeat within the interval |
