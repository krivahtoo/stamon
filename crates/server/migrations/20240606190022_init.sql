-- users table: Stores users who might receive notifications
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL,
    password TEXT NOT NULL,
    role TEXT NOT NULL DEFAULT 'admin',
    active BOOLEAN NOT NULL DEFAULT 1,
    timezone TEXT
);

-- entities table: Stores monitored entities
CREATE TABLE entities (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    active BOOLEAN NOT NULL DEFAULT 1,
    name TEXT NOT NULL,
    interval INTEGER NOT NULL,
    url TEXT,
    payload TEXT,
    monitor_type TEXT NOT NULL,
    retry INTEGER NOT NULL DEFAULT 3,
    retry_interval INTEGER NOT NULL,

    FOREIGN KEY (user_id) REFERENCES users(id)
);

-- status_logs table: Logs the status changes of entities
CREATE TABLE status_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    entity_id INTEGER NOT NULL,
    status INTEGER NOT NULL,
    message TEXT,
    time DATETIME DEFAULT CURRENT_TIMESTAMP,
    ping INTEGER,
    duration INTEGER NOT NULL,

    FOREIGN KEY (entity_id) REFERENCES entities(id)
);

-- notifications table: Stores notification logs related to status changes
CREATE TABLE notifications (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    entity_id INTEGER NOT NULL,
    status_log_id INTEGER NOT NULL,
    message TEXT,
    sent_at DATETIME DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (entity_id) REFERENCES entities(id),
    FOREIGN KEY (status_log_id) REFERENCES status_logs(id)
);

-- configuration table: Stores configuration settings
CREATE TABLE config (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    value TEXT NOT NULL,
    last_updated DATETIME DEFAULT CURRENT_TIMESTAMP
);