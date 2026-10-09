-- Public pages showing the status of chosen services
CREATE TABLE StatusPages (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    -- Public URL: /status/<slug>
    slug TEXT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    description TEXT,
    -- Unpublished pages are only visible to signed-in users
    published BOOLEAN NOT NULL DEFAULT 0,

    FOREIGN KEY (user_id) REFERENCES Users(id)
);

CREATE TABLE StatusPageServices (
    page_id INTEGER NOT NULL,
    service_id INTEGER NOT NULL,
    -- Order on the page
    position INTEGER NOT NULL,

    PRIMARY KEY (page_id, service_id),
    FOREIGN KEY (page_id) REFERENCES StatusPages(id) ON DELETE CASCADE,
    FOREIGN KEY (service_id) REFERENCES Services(id) ON DELETE CASCADE
);
