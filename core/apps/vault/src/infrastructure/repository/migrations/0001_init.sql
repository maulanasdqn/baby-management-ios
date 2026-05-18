CREATE TABLE IF NOT EXISTS milestones (
    id          TEXT PRIMARY KEY NOT NULL,
    title       TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    occurred_at INTEGER NOT NULL,
    created_at  INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS growth_logs (
    id            TEXT PRIMARY KEY NOT NULL,
    weight_grams  INTEGER,
    height_mm     INTEGER,
    notes         TEXT NOT NULL DEFAULT '',
    logged_at     INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS media_metadata (
    id              TEXT PRIMARY KEY NOT NULL,
    title           TEXT NOT NULL,
    encrypted_path  TEXT NOT NULL,
    size_bytes      INTEGER NOT NULL,
    created_at      INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS sync_state (
    entity_type TEXT NOT NULL,
    entity_id   TEXT NOT NULL,
    synced_at   INTEGER NOT NULL,
    PRIMARY KEY (entity_type, entity_id)
);
