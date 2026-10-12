+++
title = "Status pages"
description = "Share the status and uptime of your services publicly."
weight = 3
+++

Under **Status Pages**, create a page with a title, an address (slug) and the
services to show, in order. Once **published**, anyone can open it without an
account at:

```
https://stamon.example.com/status/<slug>/
```

## What visitors see

- Each service's name, current status, 30-day uptime and a bar per day.
- Upcoming [maintenance](@/docs/features/maintenance-windows.md).

They never see URLs, hosts or settings, and paused services are left out.
Unpublished pages return *not found*.
