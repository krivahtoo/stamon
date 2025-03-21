+++
title = "Install"
description = "Learn how to install and set up Stamon on your system."
weight = 2
+++

# 🚀 Installation

This guide will walk you through installing **Stamon**, a lightweight, self-hosted status monitoring tool written in Rust.

---

## 📥 Prerequisites

Before installing **Stamon**, make sure you have the following dependencies installed:

- **Docker** (for containerized deployment) → [Install Docker](https://docs.docker.com/get-docker/)
- **Rust** (for manual installation) → [Install Rust](https://www.rust-lang.org/tools/install)
- **Node.js & PNPM** (for frontend development) → [Install Node.js](https://nodejs.org/en) & [PNPM](https://pnpm.io/installation)

---

## 🐳 Install via Docker (Recommended)

The easiest way to get **Stamon** up and running is using Docker.

### **Run with Docker CLI**

```sh
docker run -it -e JWT_SECRET="hello-world" -p 3000:3000 k4htoo/stamon
```

### **Run with Docker Compose**

Create a `docker-compose.yml` file:

```yml
version: "3"
services:
  stamon:
    image: k4htoo/stamon
    environment:
      JWT_SECRET: "A_very_secure_secret"
    ports:
      - "3000:3000"
```

Then run:

```sh
docker compose up -d
```

Now visit <http://localhost:3000>.

## ⚙️ Manual Installation (Rust)

For those who prefer running **Stamon** without Docker:

### 1. Clone the Repository

```sh
git clone https://github.com/krivahtoo/stamon.git
cd stamon
```

### 2. Build and Run

```sh
cargo build --release
./target/release/stamon
```

By default, the server runs on **port 3000**.

## 🏗️ Development Setup

To set up a development environment:

### Backend

```sh
cargo run
```

### Frontend

```sh
cd frontend
pnpm install
pnpm dev
```

Then visit <http://localhost:5173>.

### 📝 Configuration

You can customize Stamon using environment variables:


|Variable       | Description                           |Default Value  |
|---------------|---------------------------------------|---------------|
|`JWT_SECRET`   | Secret key for authentication         | Required      |
|`PORT`	        | Port for the server                   | `3000`        |
|`DATABASE_URL` | Database connection URL (if using DB) | Optional      |

To set these values, create a `.env` file:

```sh
JWT_SECRET="super_secret_key"
PORT=3000
```
