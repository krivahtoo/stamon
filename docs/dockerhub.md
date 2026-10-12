# Stamon

**A lightweight, self-hosted status monitoring tool written in Rust.**

Stamon checks your websites, servers, databases and background jobs, alerts you when something goes down, and gives you public status pages to share with your users. It runs as a single binary backed by SQLite.

![Stamon dashboard](https://github.com/user-attachments/assets/13d63921-0316-4182-af01-3c0786769f8d)

[GitHub](https://github.com/krivahtoo/stamon) · [Documentation](https://stamon.org) · [Usage guide](https://stamon.org/usage/) · [Issues](https://github.com/krivahtoo/stamon/issues)

## Features

- **Many kinds of checks**: HTTP(S) with status, keyword and JSON assertions; ping; TCP ports; DNS records; TLS certificate expiry; Docker containers; PostgreSQL, MySQL/MariaDB and Redis; and push heartbeats for cron jobs and workers.
- **Fewer false alarms**: retries before a service is marked down, and an invert option for checks that should fail.
- **Alerts** via webhook, Slack, Discord, Telegram, ntfy and email when a service goes down and when it recovers, with a delivery history.
- **Maintenance windows**: planned downtime isn't checked, doesn't alert and doesn't count against uptime.
- **Public status pages** showing the status and 30-day uptime of the services you choose.
- **Uptime and latency stats**, **tags**, and **roles** (viewer, editor, admin) for teams.
- **Real-time** dashboard updates.

## Quick start

```sh
docker run -d \
  --name stamon \
  -e JWT_SECRET="$(openssl rand -base64 48)" \
  -p 3000:3000 \
  -v stamon-data:/app/data \
  k4htoo/stamon
```

Open <http://localhost:3000> and register. The first account becomes the admin. After that, registration closes and admins add users.

### Docker Compose

```yaml
services:
  stamon:
    image: k4htoo/stamon
    restart: unless-stopped
    environment:
      JWT_SECRET: "replace-with-a-long-random-secret"
    ports:
      - "3000:3000"
    volumes:
      - stamon-data:/app/data

volumes:
  stamon-data:
```

```sh
docker compose up -d
```

## Configuration

| Variable | Default | Description |
|---|---|---|
| `JWT_SECRET` | **required** | Secret used to sign login tokens. Use a long random value, e.g. `openssl rand -base64 48`. |
| `DATA_PATH` | `/app/data` | Directory for the SQLite database. |
| `ASSETS_PATH` | `/assets/` | Directory of the built web interface. |
| `LOG_RETENTION_DAYS` | `90` | Days to keep check logs and alert history; `0` keeps them forever. |
| `RUST_LOG` | | Log filter, e.g. `info` or `server=debug`. |

| | |
|---|---|
| **Port** | `3000` (publish it on another port with `-p 8080:3000`) |
| **Volume** | `/app/data` holds the database. Mount a volume so data survives container updates. |

## Monitoring Docker containers

Docker checks read container state from the Docker API. To monitor containers on the same host, mount the Docker socket (read-only is enough):

```yaml
services:
  stamon:
    image: k4htoo/stamon
    volumes:
      - stamon-data:/app/data
      - /var/run/docker.sock:/var/run/docker.sock:ro
```

> **Warning:** access to the Docker socket is equivalent to root access on the host. Only mount it if you trust everyone who can manage monitors.

A daemon exposed over TCP (`tcp://host:2375`) also works. TLS-protected daemons aren't supported yet.

## Tags

| Tag | Description |
|---|---|
| `latest` | The latest release. |
| `<version>` (e.g. `0.3.0`) | A specific release. |
| `<major>.<minor>` (e.g. `0.3`) | The latest patch of a minor version. |
| `edge` | The latest build from the `main` branch; may be unstable. |
| `sha-<commit>` | Built from a specific commit. |

Images are published for `linux/amd64` and `linux/arm64`.

## Updating

```sh
docker compose pull && docker compose up -d
```

Database migrations run automatically on startup. Keep the `/app/data` volume between updates.

## License

[MIT](https://github.com/krivahtoo/stamon/blob/main/LICENSE). Stamon is in active development; feedback and contributions are welcome on [GitHub](https://github.com/krivahtoo/stamon).
