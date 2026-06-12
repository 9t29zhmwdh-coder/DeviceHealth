CREATE TABLE IF NOT EXISTS health_snapshots (
    id               TEXT PRIMARY KEY,
    score            INTEGER NOT NULL,
    grade            TEXT NOT NULL,
    cpu_usage        REAL NOT NULL DEFAULT 0,
    memory_used_pct  REAL NOT NULL DEFAULT 0,
    process_count    INTEGER NOT NULL DEFAULT 0,
    critical         INTEGER NOT NULL DEFAULT 0,
    high             INTEGER NOT NULL DEFAULT 0,
    medium           INTEGER NOT NULL DEFAULT 0,
    low              INTEGER NOT NULL DEFAULT 0,
    info             INTEGER NOT NULL DEFAULT 0,
    uptime_seconds   INTEGER NOT NULL DEFAULT 0,
    timestamp_ts     INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_snapshots_ts ON health_snapshots(timestamp_ts DESC);

CREATE TABLE IF NOT EXISTS app_settings (
    key   TEXT PRIMARY KEY,
    value TEXT NOT NULL
);
