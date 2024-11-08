-- users table: Stores users who might receive notifications
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL,
    password TEXT NOT NULL,
    role TEXT NOT NULL DEFAULT 'admin',
    active BOOLEAN NOT NULL DEFAULT 1,
    timezone TEXT
);

-- services table: Stores monitored service
CREATE TABLE services (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    active BOOLEAN NOT NULL DEFAULT 1,
    name TEXT NOT NULL,
    interval INTEGER NOT NULL,
    url TEXT NOT NULl,
    payload TEXT,
    timeout INTEGER NOT NULL DEFAULT 10,
    last_status INTEGER NOT NULL DEFAULT 0,
    service_type TEXT NOT NULL,
    retry INTEGER NOT NULL DEFAULT 3,
    retry_interval INTEGER NOT NULL,
    invert BOOLEAN NOT NULL DEFAULT 0,
    expected_code INTEGER,
    expected_payload TEXT,

    FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE INDEX IF NOT EXISTS svc_idx ON services(last_status);

-- logs table: Logs the status changes of services
CREATE TABLE logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    service_id INTEGER NOT NULL,
    status INTEGER NOT NULL,
    message TEXT,
    time DATETIME DEFAULT CURRENT_TIMESTAMP,
    duration INTEGER NOT NULL,

    FOREIGN KEY (service_id) REFERENCES services(id) ON DELETE CASCADE
);

-- notifications table: Stores notification logs related to status changes
CREATE TABLE notifications (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    service_id INTEGER NOT NULL,
    log_id INTEGER NOT NULL,
    message TEXT,
    sent_at DATETIME DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (service_id) REFERENCES services(id) ON DELETE CASCADE,
    FOREIGN KEY (log_id) REFERENCES logs(id)
);

-- configuration table: Stores configuration settings
CREATE TABLE configs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    value TEXT NOT NULL,
    category TEXT,
    last_updated DATETIME DEFAULT CURRENT_TIMESTAMP
);