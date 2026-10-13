+++
title = "Backups and data"
description = "Where Stamon keeps data, how long it keeps it, and how to back it up."
weight = 5
+++

## Where data lives

Everything is in one SQLite database, `stamon.db`, in `DATA_PATH` (`/app/data`
in Docker). The database runs in WAL mode, so `stamon.db-wal` and
`stamon.db-shm` files sit beside it while Stamon runs.

## Back up

The simplest safe backup is to stop Stamon and copy the directory. This uses
the volume attached to the Stamon container, whatever Compose named it:

```sh
docker compose stop stamon
docker run --rm --volumes-from "$(docker compose ps -aq stamon)" -v "$PWD":/backup alpine \
  tar czf /backup/stamon-$(date +%F).tar.gz -C /app/data .
docker compose start stamon
```

To back up without stopping, use SQLite's online backup, which produces a
consistent copy:

```sh
sqlite3 /path/to/data/stamon.db ".backup 'stamon-backup.db'"
```

Don't copy `stamon.db` alone while Stamon is running; recent changes may still
be in the WAL file.

To restore, stop Stamon, replace the contents of `DATA_PATH` with the backup,
and start it again.

## Retention {#retention}

Check logs and alert history are kept for `LOG_RETENTION_DAYS` days (default
90; `0` keeps them forever). Older entries are deleted every hour.

## How uptime is calculated {#uptime}

**Uptime** is the share of *up* results among up and down ones. Pending, failed
and maintenance results don't count.
