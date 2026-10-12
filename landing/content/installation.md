+++
title = "Install"
description = "Installing stamon"
weight = 2
+++

This guide will walk you through installing **stamon**, a lightweight, self-hosted status monitoring tool written in Rust.

---

# 📥 Prerequisites

- **Docker** (recommended) → [Install Docker](https://docs.docker.com/get-docker/)
- Or, to build from source: **Rust** → [Install Rust](https://www.rust-lang.org/tools/install), plus **Node.js & PNPM** for the frontend → [Install Node.js](https://nodejs.org/en) & [PNPM](https://pnpm.io/installation)

---

# 🐳 Install via Docker (Recommended)

## **Run with Docker CLI**

```sh
docker run -d -e JWT_SECRET="change-me" -p 3000:3000 -v stamon-data:/app/data k4htoo/stamon
```

The `stamon-data` volume keeps the database when the container is replaced. Generate a strong `JWT_SECRET`, for example with `openssl rand -base64 48`.

## **Run with Docker Compose**

Create a `compose.yml` file:

```yml
services:
  stamon:
    image: k4htoo/stamon
    environment:
      JWT_SECRET: "A_very_secure_secret"
    ports:
      - "3000:3000"
    volumes:
      - stamon-data:/app/data
      # Only needed to monitor Docker containers; see the usage guide.
      # - /var/run/docker.sock:/var/run/docker.sock:ro

volumes:
  stamon-data:
```

Then run:

```sh
docker compose up -d
```

Now visit <http://localhost:3000>.

# ⚙️ Manual Installation (Rust)

## 1. Clone the Repository

```sh
git clone https://github.com/krivahtoo/stamon.git
cd stamon
```

## 2. Build the Frontend

```sh
cd frontend
pnpm install
pnpm build
cd ..
```

The built frontend is written to `frontend/build/`.

## 3. Build and Run the Server

```sh
cargo build --release
DATA_PATH=./data JWT_SECRET="change-me" ASSETS_PATH=./frontend/build ./target/release/server
```

The server creates the database in `DATA_PATH` and listens on **port 3000**.

# 👤 First Run {#first-run}

Open Stamon and register. The first account becomes the **admin**; after that, registration is closed.

Admins add more users through the API for now (a dashboard page is planned). Sign in to get a token, then create the user with a role of `viewer`, `editor` or `admin`:

```sh
TOKEN=$(curl -s -H 'Content-Type: application/json' \
  -d '{"username":"admin","password":"your-password"}' \
  http://localhost:3000/api/login | jq -r .token)

curl -H "Authorization: Bearer $TOKEN" -H 'Content-Type: application/json' \
  -d '{"username":"alice","password":"a-strong-password","role":"editor"}' \
  http://localhost:3000/api/users
```

See [Users and roles](/usage/#users-and-roles) for what each role can do.

# 📝 Configuration

Stamon is configured with environment variables:

| Variable             | Description                                                        | Default                              |
|----------------------|--------------------------------------------------------------------|--------------------------------------|
| `JWT_SECRET`         | Secret used to sign login tokens                                   | Required                             |
| `DATA_PATH`          | Directory for the SQLite database                                  | Required (`/app/data` in Docker)     |
| `ASSETS_PATH`        | Directory of the built frontend                                    | `assets` (`/assets/` in Docker)      |
| `LOG_RETENTION_DAYS` | Days to keep check logs and alert history; `0` keeps them forever  | `90`                                 |
| `RUST_LOG`           | Log filter, e.g. `info` or `server=debug`                          | None                                 |

The server always listens on port **3000**. With Docker, publish it on another port with `-p 8080:3000`.

# 🏗️ Development Setup

`.cargo/config.toml` sets a development environment (`DATA_PATH=./data`, a development `JWT_SECRET`, and so on), so you only need:

## Backend

```sh
cargo run
```

## Frontend

```sh
cd frontend
pnpm install
pnpm dev
```

Then visit <http://localhost:5173>.

Run the tests with `cargo test --workspace`, and the frontend checks with `pnpm lint` and `pnpm check`.
