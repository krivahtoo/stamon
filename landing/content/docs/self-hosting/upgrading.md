+++
title = "Upgrading"
description = "Move to a new Stamon release."
weight = 4
+++

Read the [changelog](@/changelog.md) before upgrading.

{% <devlab.steps> %}
1. **Back up the data.** See [Backups and data](@/docs/self-hosting/backups-and-data.md).

2. **Pull the new image and recreate the container.**

   ```sh
   docker pull k4htoo/stamon
   docker compose up -d
   ```

   With `docker run`, stop and remove the old container, then start a new one
   with the same volume and environment.

3. **Check the logs.** Database migrations run automatically on start:

   ```sh
   docker compose logs -f stamon
   ```
{% </devlab.steps> %}

## Image tags

| Tag | Follows |
|---|---|
| `latest` | The newest release |
| `0.3.0`, `0.3` | A release, or a release line |
| `edge` | The main branch; may be unstable |

Pin a version tag in production so upgrades happen when you choose.

## Downgrading

Migrations only go forward. To go back to an older version, restore the backup
you made before upgrading.
