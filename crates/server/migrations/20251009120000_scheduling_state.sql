-- Unix timestamp of the next scheduled check; 0 means due immediately
ALTER TABLE Services
ADD next_run_at INTEGER NOT NULL DEFAULT 0;

-- Failed checks in a row, used to retry before marking a service down
ALTER TABLE Services
ADD consecutive_failures INTEGER NOT NULL DEFAULT 0;

CREATE INDEX IF NOT EXISTS svc_next_run_idx ON Services(active, next_run_at);
