+++
title = "Maintenance windows"
description = "Pause checks and alerts during planned work."
weight = 2
+++

Schedule planned work under **Maintenance**, for chosen services or all of them.

While a window is active, covered services:

- aren't checked, and show as *Maintenance*;
- send no alerts;
- don't count against uptime.

If a service is still down when the window ends, it's retried and then alerts
as usual. Upcoming maintenance is also shown on
[status pages](@/docs/features/status-pages.md).
