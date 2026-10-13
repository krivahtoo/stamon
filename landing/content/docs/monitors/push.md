+++
title = "Push"
description = "Let cron jobs, backups and workers report in."
weight = 8
+++

For cron jobs, backups and workers that can't be reached from Stamon, the job
reports in instead. Each push monitor has a secret URL, shown when you create
it and on its page to editors and admins:

```
https://stamon.example.com/api/push/<token>
```

Call it with `GET` or `POST` at least once per interval. If no heartbeat
arrives within the interval plus the **grace period**, the monitor goes down.

## Query parameters

| Parameter | Meaning |
|---|---|
| `status` | `up` (default) or `down` |
| `msg` | A message stored with the heartbeat |
| `ping` | A response time to record, in milliseconds |

## Example

At the end of a backup script:

```sh
if ./backup.sh; then
  curl -fsS "https://stamon.example.com/api/push/<token>"
else
  curl -fsS "https://stamon.example.com/api/push/<token>?status=down&msg=backup%20failed"
fi
```

Treat the URL like a password: anyone who has it can report the job's status.
Viewers can't see push tokens.
