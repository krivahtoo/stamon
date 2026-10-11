-- Unix timestamp of the last heartbeat a push monitor received
ALTER TABLE Services
ADD last_push_at INTEGER;
