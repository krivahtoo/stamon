+++
title = "Quick tour"
description = "Add a monitor, get an alert and publish a status page."
weight = 3
+++

This tour assumes Stamon is [installed](@/docs/getting-started/installation.md)
and you have [signed in](@/docs/getting-started/first-run.md) as the admin.

{% <devlab.steps> %}
1. **Add a monitor.** Under **Monitors**, add a service. Pick a check type, such
   as [HTTP(S)](@/docs/monitors/http.md) with your website's URL, and an
   interval. The dashboard updates as soon as the first check runs.

2. **Avoid false alarms.** Set **retries** so one slow response doesn't mark the
   service down. See [shared settings](@/docs/monitors/_index.md#settings).

3. **Get alerted.** Under **Notification Providers**, add a channel such as Slack
   or email and press **Test**. Turn on **All services**, or pick the channel on
   each service. See [Alerts](@/docs/features/alerts.md).

4. **Share the status.** Under **Status Pages**, create a page, choose the
   services to show and publish it. See [Status pages](@/docs/features/status-pages.md).

5. **Invite your team.** Add viewers and editors. See
   [Users and roles](@/docs/features/users-and-roles.md).
{% </devlab.steps> %}

Before you rely on Stamon, read [Security](@/docs/self-hosting/security.md) and
[Backups and data](@/docs/self-hosting/backups-and-data.md).
