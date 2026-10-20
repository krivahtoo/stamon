# Stamon frontend

The Stamon dashboard and public status pages, built with SvelteKit and served by the Rust server as static files.

## Developing

```bash
pnpm install
pnpm dev
```

The dev server runs on <http://localhost:5173> and proxies `/api` and `/ws` to the backend on port 3000, so start the backend too (`cargo run` from the repository root).

## Building

```bash
pnpm build
```

The static site is written to `build/`. The server serves it from `ASSETS_PATH`, which `.cargo/config.toml` points at `./frontend/build/` during development.

## Checking

```bash
pnpm lint    # prettier and eslint
pnpm check   # svelte-check
```

`svelte-check` currently reports some existing type errors, mostly in the generated `src/lib/components/ui` components; new code shouldn't add to them.

## Layout

- `src/routes/(dashboard)/`: signed-in pages (services, maintenance, status pages, notification providers).
- `src/routes/status/[slug]/`: public status pages, open without signing in.
- `src/routes/login/`, `src/routes/register/`: sign-in and first-run setup.
- `src/lib/store/`: shared state (signed-in user, services, live updates over WebSocket).
- `src/lib/utils.js`: `cfetch` for API calls, and `pushUrl` for push monitor URLs.
- `src/lib/components/ui/`: shadcn-svelte components.
