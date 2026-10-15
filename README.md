<div align="center">
  <h1>
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://github.com/user-attachments/assets/1b384c38-7d94-4d42-90c4-098cfddf61af">
      <source media="(prefers-color-scheme: light)" srcset="https://github.com/user-attachments/assets/a7d933c6-e07a-43f6-989f-8f2224f4fdee">
      <img alt="Shows a black logo in light color mode and a white one in dark color mode." src="https://github.com/user-attachments/assets/a7d933c6-e07a-43f6-989f-8f2224f4fdee">
    </picture>
  </h1>

  *A lightweight self-hosted status monitoring tool, written in Rust.*

  [![CI](https://github.com/krivahtoo/stamon/actions/workflows/ci.yml/badge.svg)](https://github.com/krivahtoo/stamon/actions/workflows/ci.yml)
  ![Docker Image Version](https://img.shields.io/docker/v/k4htoo/stamon?logo=docker&sort=semver)
  ![Docker Pulls](https://img.shields.io/docker/pulls/k4htoo/stamon)
  ![GitHub last commit](https://img.shields.io/github/last-commit/krivahtoo/stamon?style=flat&logo=git&logoColor=%23F05032)
  ![GitHub License](https://img.shields.io/github/license/krivahtoo/stamon)
</div>


> [!NOTE]
> Stamon is in active development. You are welcome to try it out and provide feedback! ;-)

![2024-09-19-180624_1366x665_scrot](https://github.com/user-attachments/assets/13d63921-0316-4182-af01-3c0786769f8d)

## ✨ Features

- **Many kinds of checks**: HTTP(S) with status, keyword and JSON assertions; ping; TCP ports; DNS records; TLS certificate expiry; Docker containers; PostgreSQL, MySQL/MariaDB and Redis; and push heartbeats for cron jobs and workers.
- **Fewer false alarms**: retries before a service is marked down, and an invert option for checks that should fail.
- **Alerts**: webhook, Slack, Discord, Telegram, ntfy and email, sent when a service goes down and when it recovers, with a delivery history.
- **Maintenance windows**: planned downtime isn't checked, doesn't alert and doesn't count against uptime.
- **Public status pages**: share a page showing the status and 30-day uptime of the services you choose.
- **Uptime and latency stats**, **tags** for organising services, and **roles** (viewer, editor, admin) for teams.
- **Real-time** updates in the dashboard, and a **lightweight** single binary backed by SQLite.

See the [usage guide](https://stamon.org/usage/) for how each feature works.

## 🚀 Quick Start

Try it out and [open an issue](https://github.com/krivahtoo/stamon/issues) if you find a bug or want a feature added.

### 🚢 Docker

#### `docker run`

```sh
docker run -d -e JWT_SECRET="change-me" -p 3000:3000 -v stamon-data:/app/data k4htoo/stamon
```

#### `docker compose`

```yaml
services:
  stamon:
    image: k4htoo/stamon

    environment:
      JWT_SECRET: "A_very_secure_secret"
    ports:
      - "3000:3000"
    volumes:
      - stamon-data:/app/data

volumes:
  stamon-data:
```
Then, start the container:
```sh
docker compose up stamon
```

Visit http://localhost:3000 and register. The first account becomes the admin; after that, registration closes and admins add users (see the [usage guide](https://stamon.org/usage/#users-and-roles)).

The volume keeps the database when the container is replaced. Generate a strong `JWT_SECRET`, for example with `openssl rand -base64 48`.

### ⚙️ Configuration

| Variable | Default | Description |
|---|---|---|
| `JWT_SECRET` | (required) | Secret used to sign login tokens. |
| `DATA_PATH` | (required; `/app/data` in Docker) | Directory for the SQLite database. |
| `ASSETS_PATH` | `assets` (`/assets/` in Docker) | Directory of the built frontend. |
| `LOG_RETENTION_DAYS` | `90` | Days to keep check logs and alert history; `0` keeps them forever. |
| `RUST_LOG` | | Log filter, e.g. `info` or `server=debug`. |

The server listens on port `3000`; publish it on another port with `-p 8080:3000`.

### 🐳 Monitoring Docker containers

Docker checks read container state from the Docker API. When Stamon itself runs in Docker, give it the host's socket (read-only is enough):

```yaml
services:
  stamon:
    image: k4htoo/stamon
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock:ro
```

Access to the Docker socket is equivalent to root access on the host, so only mount it if you trust everyone who can manage monitors. A daemon exposed over TCP (`tcp://host:2375`) works too; TLS-protected daemons aren't supported yet.

## 🏗️ Development

The workspace has a server crate (`crates/server`), libraries for checks (`crates/checks`), alerts (`crates/alerts`) and TLS expiry (`crates/ssl-exp`), and a SvelteKit frontend (`frontend`).

To start the frontend:
```shell
cd ./frontend
pnpm install
pnpm dev
```

Then in another terminal start the backend. `.cargo/config.toml` sets the development environment (`DATA_PATH=./data`, a development `JWT_SECRET`, and so on):

```shell
cargo run
```

Then visit http://localhost:5173.

To run the checks:

```shell
cargo test --workspace
cd frontend && pnpm lint && pnpm check
```

## 📝 Contributing

Thank you for considering contributing to Stamon! Contributions are welcome, including:

  - **New Issues**: Feature requests, bug reports, questions, ideas, and more.
  - **Pull Requests**: Documentation updates, code improvements, new features, etc.

## 👷 Contributors
<a href="https://github.com/krivahtoo/stamon/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=krivahtoo/stamon" alt="Contributors to krivahtoo/stamon" />
</a>

Made with [contrib.rocks](https://contrib.rocks).

## 🛣️ Roadmap

- [x] Implement notification system for alerts
  - [x] Integration with third-party services (e.g., Telegram, Slack, email notifications)
- [x] Add more monitoring types (e.g. DNS, TCP, docker)
- [ ] Add support for custom alert thresholds
- [ ] Manage users from the dashboard (currently through the API)
- [ ] Edit the check settings of existing monitors from the dashboard
- [ ] Recurring maintenance windows
- [ ] TLS for Docker daemons and Redis checks


## 📜 License
This project is licensed under the [MIT license](./LICENSE).
