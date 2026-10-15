+++
title = "Docker container"
description = "Check that a container is running and healthy."
weight = 6
+++

Name the container (or its id). The monitor is up while the container is
running and not reported unhealthy. While a container is stopped or restarting,
it is down. Once it is running, a health check that is still *starting* counts
as up; only *unhealthy* counts as down. Use [retries](@/docs/monitors/_index.md#settings)
to ride out short restarts.

## Connect to Docker

Stamon talks to the Docker API at `unix:///var/run/docker.sock` by default, or
at a `tcp://host:2375` address. TLS-protected daemons aren't supported yet.

A daemon on `tcp://host:2375` has no authentication: anyone who can reach that
port controls Docker, and through it the host. Only use it on a trusted,
isolated network, never one exposed to the internet.

When Stamon runs in Docker, mount the host's socket into its container:

```yml
services:
  stamon:
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock:ro
```

{% <devlab.callout type="warning" title="The Docker socket is root access"> %}
Access to the Docker socket is equivalent to root access on the host. Only
mount it if you trust everyone who can manage monitors. See
[Security](@/docs/self-hosting/security.md).
{% </devlab.callout> %}
