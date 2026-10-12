+++
title = "Installation"
description = "Run Stamon with Docker, Docker Compose or from source."
weight = 1
aliases = ["/installation/", "/docs/installation/"]
+++

Stamon ships as a Docker image, `k4htoo/stamon`. You can also build it from
source.

## Requirements

- **Docker** (recommended): [install Docker](https://docs.docker.com/get-docker/).
- To build from source: [Rust](https://www.rust-lang.org/tools/install),
  [Node.js](https://nodejs.org/en) and [pnpm](https://pnpm.io/installation).

## Run Stamon

{% <devlab.tabs sync="install-method" label="Installation method"> %}
{% <devlab.tab name="Docker" value="docker" selected={true}> %}
```sh
docker run -d -e JWT_SECRET="change-me" -p 3000:3000 -v stamon-data:/app/data k4htoo/stamon
```
{% </devlab.tab> %}
{% <devlab.tab name="Docker Compose" value="compose"> %}
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
      # Only needed to monitor Docker containers; see the Docker monitor page.
      # - /var/run/docker.sock:/var/run/docker.sock:ro

volumes:
  stamon-data:
```

Then run:

```sh
docker compose up -d
```
{% </devlab.tab> %}
{% <devlab.tab name="From source" value="source"> %}
{% <devlab.steps> %}
1. **Clone the repository.**

   ```sh
   git clone https://github.com/krivahtoo/stamon.git
   cd stamon
   ```

2. **Build the frontend.** The output is written to `frontend/build/`.

   ```sh
   cd frontend
   pnpm install
   pnpm build
   cd ..
   ```

3. **Build and run the server.**

   ```sh
   cargo build --release
   DATA_PATH=./data JWT_SECRET="change-me" ASSETS_PATH=./frontend/build ./target/release/server
   ```
{% </devlab.steps> %}
{% </devlab.tab> %}
{% </devlab.tabs> %}

Stamon listens on port **3000**. Open <http://localhost:3000> and continue with
[First run](@/docs/getting-started/first-run.md).

{% <devlab.callout type="warning" title="Keep the data and the secret"> %}
The `stamon-data` volume (or `DATA_PATH`) holds the database, so keep it when
you replace the container. Generate a strong `JWT_SECRET`, for example with
`openssl rand -base64 48`.
{% </devlab.callout> %}

All settings are listed in [Configuration](@/docs/self-hosting/configuration.md).
