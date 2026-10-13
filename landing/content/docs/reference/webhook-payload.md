+++
title = "Alert payloads"
description = "What each notification channel receives."
weight = 2
+++

## Webhook {#webhook}

Webhook channels receive a `POST` with a JSON body, plus any headers you
configured:

```json
{
  "service_id": 7,
  "service_name": "API",
  "target": "https://api.example.com",
  "status": "down",
  "message": "Unexpected status code: 500",
  "time": "2026-10-09T12:00:00Z",
  "title": "API is down",
  "text": "🔴 API is down\nUnexpected status code: 500\nTarget: https://api.example.com\nTime: 2026-10-09 12:00:00 UTC"
}
```

| Field | Description |
|---|---|
| `service_id`, `service_name` | The service |
| `target` | Its URL, host or container; `null` for push monitors |
| `status` | `down` or `up` |
| `message` | Why the check failed; may be `null` |
| `time` | When it happened, in RFC 3339 |
| `title` | `<name> is down` or `<name> is back up` |
| `text` | A ready-to-post message: title, message, target and time |

## Other channels

| Channel | Sends |
|---|---|
| Slack | `{"text": text}` to the incoming webhook |
| Discord | `{"content": text}` to the webhook |
| Telegram | `text` to the chat, through the bot API |
| ntfy | The message, target and time, with the title as `Title`, priority `high` when down, and a red or green circle tag |
| Email | Subject `[Stamon] <title>`, with the title, message, target and time as plain text |

## Delivery

- A delivery fails if the receiver doesn't answer with a 2xx status within 10
  seconds. Redirects aren't followed, so a 3xx response counts as a failure.
- Failed deliveries are tried 3 times in all, 2 and 4 seconds apart.
- The alert **History** records one entry per delivery: its final result after
  all attempts, with the last error for failures. Individual failed attempts
  only appear in the server log.
