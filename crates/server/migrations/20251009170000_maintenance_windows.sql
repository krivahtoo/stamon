-- Planned downtime: covered services aren't checked and don't alert
CREATE TABLE MaintenanceWindows (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    starts_at DATETIME NOT NULL,
    ends_at DATETIME NOT NULL,
    -- Covers every service, not only the ones linked to it
    apply_to_all BOOLEAN NOT NULL DEFAULT 0,

    FOREIGN KEY (user_id) REFERENCES Users(id)
);

CREATE TABLE MaintenanceServices (
    window_id INTEGER NOT NULL,
    service_id INTEGER NOT NULL,

    PRIMARY KEY (window_id, service_id),
    FOREIGN KEY (window_id) REFERENCES MaintenanceWindows(id) ON DELETE CASCADE,
    FOREIGN KEY (service_id) REFERENCES Services(id) ON DELETE CASCADE
);
