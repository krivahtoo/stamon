+++
title = "Configuration"
description = "Environment variables and network settings."
weight = 1
+++

Stamon is configured with environment variables.

| Variable | Description | Default |
|---|---|---|
| `JWT_SECRET` | Secret used to sign login tokens | Required |
| `DATA_PATH` | Directory for the SQLite database; created if missing | Required (`/app/data` in the Docker image) |
| `ASSETS_PATH` | Directory of the built frontend | `assets` (`/assets/` in the Docker image) |
| `LOG_RETENTION_DAYS` | Days to keep check logs and alert history; `0` keeps them forever | `90` |
| `RUST_LOG` | Log filter, e.g. `info` or `server=debug` | None |

## Network

- The server listens on port **3000** on all interfaces. The port can't be
  changed; with Docker, publish it on another port with `-p 8080:3000`.
- Stamon must be served from the root of its domain (`https://status.example.com/`),
  not a sub-path like `/stamon/`.
- For anything beyond localhost, serve it over HTTPS. See
  [Reverse proxy](@/docs/self-hosting/reverse-proxy.md).

## Sign-in tokens

Tokens are signed with `JWT_SECRET` and last 60 minutes. Changing the secret
signs everyone out.
