+++
title = "Database"
description = "Check that PostgreSQL, MySQL/MariaDB or Redis answers queries."
weight = 7
+++

Pick the engine and give the host, port (the engine's default otherwise),
credentials and database (a number for Redis). Stamon connects and runs
`SELECT 1`, or `PING` for Redis.

| Engine | Query | TLS |
|---|---|---|
| PostgreSQL | `SELECT 1` | **Require TLS** available; otherwise used when the server offers it |
| MySQL / MariaDB | `SELECT 1` | **Require TLS** available; otherwise used when the server offers it |
| Redis | `PING` | Not supported yet |

Use a read-only account. The password is hidden from the API once saved; it
shows as `********`.
