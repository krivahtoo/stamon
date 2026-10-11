-- Where alerts are sent. Type-specific settings live in `config` JSON, tagged by `type`.
CREATE TABLE NotificationChannels (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    active BOOLEAN NOT NULL DEFAULT 1,
    -- Receives alerts for every service, not only the ones linked to it
    apply_to_all BOOLEAN NOT NULL DEFAULT 0,
    config TEXT NOT NULL,
    channel_type TEXT GENERATED ALWAYS AS (json_extract(config, '$.type')) VIRTUAL,

    FOREIGN KEY (user_id) REFERENCES Users(id)
);

-- Channels that receive a service's alerts
CREATE TABLE ServiceChannels (
    service_id INTEGER NOT NULL,
    channel_id INTEGER NOT NULL,

    PRIMARY KEY (service_id, channel_id),
    FOREIGN KEY (service_id) REFERENCES Services(id) ON DELETE CASCADE,
    FOREIGN KEY (channel_id) REFERENCES NotificationChannels(id) ON DELETE CASCADE
);

-- The old table was never written to; it becomes a delivery history.
DROP TABLE Notifications;

CREATE TABLE Notifications (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    -- NULL for test alerts
    service_id INTEGER,
    channel_id INTEGER,
    title TEXT NOT NULL,
    message TEXT,
    -- 'sent' or 'failed'
    status TEXT NOT NULL,
    error TEXT,
    sent_at DATETIME DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (service_id) REFERENCES Services(id) ON DELETE CASCADE,
    FOREIGN KEY (channel_id) REFERENCES NotificationChannels(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS notifications_sent_idx ON Notifications(sent_at);
