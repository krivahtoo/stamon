+++
title = "Security"
description = "Protect your Stamon server and the secrets it holds."
weight = 3
+++

## Secrets

- Use a long random `JWT_SECRET`, for example `openssl rand -base64 48`. Anyone
  with it can sign in as any user.
- The database in `DATA_PATH` holds monitor settings, including database
  passwords, HTTP headers and channel credentials. Protect it and its backups.
- In the API, database passwords and HTTP header values show as `********`,
  and push tokens do too for viewers. Admins can read channel credentials.
- Treat push URLs like passwords: anyone with one can report that job's status.

## Network exposure

- Serve Stamon over HTTPS; see [Reverse proxy](@/docs/self-hosting/reverse-proxy.md).
- The live updates feed at `/ws` requires signing in, like the API. It closes
  when the sign-in expires.
- [Status pages](@/docs/features/status-pages.md) are public once published,
  but never show URLs, hosts or settings.

## Docker

{% <devlab.callout type="warning" title="The Docker socket is root access"> %}
Mounting `/var/run/docker.sock` for [Docker monitors](@/docs/monitors/docker.md)
gives Stamon root-equivalent access to the host. Only mount it if you trust
everyone who can manage monitors. Mounting it read-only (`:ro`) doesn't help:
it only stops the file being replaced, and the Docker API behind it still
allows every operation.
{% </devlab.callout> %}

The image runs as root inside the container.

## Accounts

- Registration closes after the first account, which becomes the admin.
- Give people the lowest [role](@/docs/features/users-and-roles.md) they need.
- Deactivating a user takes effect on their next request.
