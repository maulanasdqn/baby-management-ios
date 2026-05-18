CREATE TABLE IF NOT EXISTS feed_logs (
    id                TEXT PRIMARY KEY NOT NULL,
    feed_type         TEXT NOT NULL,
    amount_ml         INTEGER,
    duration_minutes  INTEGER,
    side              TEXT,
    notes             TEXT NOT NULL DEFAULT '',
    logged_at         INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS sleep_logs (
    id          TEXT PRIMARY KEY NOT NULL,
    start_time  INTEGER NOT NULL,
    end_time    INTEGER NOT NULL,
    notes       TEXT NOT NULL DEFAULT ''
);

CREATE TABLE IF NOT EXISTS diaper_logs (
    id           TEXT PRIMARY KEY NOT NULL,
    diaper_type  TEXT NOT NULL,
    notes        TEXT NOT NULL DEFAULT '',
    logged_at    INTEGER NOT NULL
);
