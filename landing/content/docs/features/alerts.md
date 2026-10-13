+++
title = "Alerts"
description = "Send alerts to webhook, Slack, Discord, Telegram, ntfy and email."
weight = 1
+++

Alerts are sent when a service is confirmed **down** (after its retries) and
when it comes back **up**. Retries, maintenance and errors in Stamon itself
don't send alerts.

## Channels {#channels}

Admins add channels under **Notification Providers**:

| Channel | Settings |
|---|---|
| **Webhook** | URL and optional headers; receives a JSON POST (see [Webhook payload](@/docs/reference/webhook-payload.md)) |
| **Slack** | Incoming webhook URL |
| **Discord** | Webhook URL |
| **Telegram** | Bot token (from @BotFather) and chat id |
| **ntfy** | Server (default `https://ntfy.sh`), topic and optional access token |
| **Email** | SMTP host, port, security (STARTTLS, TLS or none), optional username and password, sender and recipients |

Channel settings are only visible to admins.

## Choose who gets alerts

- Pick channels for each service when creating it or on its page.
- Turn on **All services** for a channel to receive every alert.

## Test and history

- **Test** sends a sample alert right away.
- Each alert is tried up to 3 times in all (2 retries). The **History** list
  shows the final result of each delivery, with the error if it failed.
- Alerts queued for a channel that has since been deactivated or deleted are
  skipped and don't appear in History.
