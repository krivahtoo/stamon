+++
title = "Development setup"
description = "Run Stamon from source with live reload."
weight = 1
+++

## Requirements

- Rust (stable; the workspace uses edition 2024)
- Node.js 18 and pnpm 9

## Run it

`.cargo/config.toml` sets a development environment (`DATA_PATH=./data`, a
development `JWT_SECRET` and verbose logging), so you only need:

{% <devlab.tabs label="Development servers"> %}
{% <devlab.tab name="Backend" value="backend" selected={true}> %}
```sh
cargo run
```

The API listens on <http://localhost:3000>.
{% </devlab.tab> %}
{% <devlab.tab name="Frontend" value="frontend"> %}
```sh
cd frontend
pnpm install
pnpm dev
```

Then visit <http://localhost:5173>. In development the frontend calls the API
on port 3000 with a bearer token, and the server allows this origin.
{% </devlab.tab> %}
{% </devlab.tabs> %}

## Checks

Run these before opening a pull request:

```sh
cargo fmt --check
cargo clippy --workspace
cargo test --workspace

cd frontend
pnpm lint
pnpm check
```

CI runs `cargo check`, `cargo fmt --check`, `cargo clippy` and the frontend
build.

## Pull requests

- Use [Conventional Commits](https://www.conventionalcommits.org/), such as
  `feat: add an SNMP check` or `fix: handle empty DNS answers`. Release notes
  are generated from them.
- Update the docs in the same pull request; see [Writing docs](@/docs/development/writing-docs.md).
