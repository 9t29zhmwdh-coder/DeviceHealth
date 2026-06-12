CREATE TABLE IF NOT EXISTS findings (
    id              TEXT PRIMARY KEY,
    snapshot_id     TEXT NOT NULL,
    kind            TEXT NOT NULL,
    severity        TEXT NOT NULL,
    title           TEXT NOT NULL,
    description     TEXT NOT NULL DEFAULT '',
    affected_item   TEXT NOT NULL DEFAULT '',
    recommendation  TEXT NOT NULL DEFAULT '',
    timestamp_ts    INTEGER NOT NULL,
    FOREIGN KEY (snapshot_id) REFERENCES health_snapshots(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_findings_snapshot  ON findings(snapshot_id);
CREATE INDEX IF NOT EXISTS idx_findings_severity  ON findings(severity);
