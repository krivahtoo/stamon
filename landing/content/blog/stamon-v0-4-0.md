+++
title = "Stamon v0.4.0: a public demo and real settings"
description = "Stamon 0.4.0 adds a demo mode and a hosted demo, real account, appearance and general settings, a new website and leaner Docker builds."
date = 2026-10-18

[taxonomies]
tags = ["release"]
authors = ["krivahtoo"]

[extra]
release = "v0.4.0"
+++

Stamon 0.4.0 is a smaller release than 0.3.0. It makes Stamon easier to try
before you install it, replaces the placeholder settings pages with real ones,
and tidies up how Stamon is built and published.

<!-- more -->

## Try it in the browser

There is now a public demo at [demo.stamon.org](https://demo.stamon.org). Sign
in as `demo` / `demo1234` (the login page fills this in for you) to explore a
dashboard with sample services, a day of check history, a maintenance window
and a [status page](https://demo.stamon.org/status/demo).

The demo runs on the new `DEMO_MODE` setting, which makes an instance safe for
anyone to use:

- An empty database is seeded with sample data and a shared **editor** account.
  There is no admin account, so visitors can't manage users.
- Checks only reach **public addresses**. This is enforced when a service is
  saved, after DNS resolution when the check runs, and on HTTP redirects.
- Docker and database checks are turned off, the minimum interval is 60
  seconds and the demo is capped at 50 services.
- Alerts are recorded in the delivery history but never sent, and
  registration, test alerts and account changes are refused.

The demo resets every hour, so feel free to break things. You can run your own
sandbox the same way by setting `DEMO_MODE=true`; see
[configuration](@/docs/self-hosting/configuration.md).

## Real settings

The settings pages were placeholders until now. They are replaced by three
sections, reachable from the new **Settings** item in the sidebar:

- **Account**: your timezone and password. Maintenance windows and alert times
  are shown in your timezone.
- **Appearance**: light, dark or system theme.
- **General** (admins only): log retention and the defaults for new services,
  plus read-only instance information such as the version and data directory.

General settings are stored in the database and take effect without a restart.
A saved retention value overrides `LOG_RETENTION_DAYS`, and the New Service form
now starts from your saved defaults.

## A new website

[stamon.org](https://stamon.org) moved to the
[DevLab](https://codeberg.org/RiPetitor/devlab-theme) theme. The install, usage
and changelog pages are now part of a [docs](@/docs/_index.md) section with a
sidebar and table of contents, and there is this blog, with an Atom feed. Old
URLs redirect to their new homes.

## Build and release

- The Docker build context dropped from over 300MB to under 2MB. It no longer
  copies the host's `node_modules` into the image, which could bring in native
  binaries built for the wrong platform.
- Release commits are built once instead of twice, and the `edge` tag still
  moves on releases.
- The [Docker Hub page](https://hub.docker.com/r/k4htoo/stamon) now gets its
  overview from the repository on every release.

## Upgrade

With Docker, pull the new image and recreate the container:

```sh
docker pull k4htoo/stamon
docker compose up -d
```

This release adds a database migration, which runs on startup. As always, keep
the `stamon-data` volume and take a [backup](@/docs/self-hosting/backups-and-data.md)
first. See [upgrading](@/docs/self-hosting/upgrading.md) for more, and the
[changelog](@/changelog.md) for every change in this release.
