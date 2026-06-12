CREATE TABLE IF NOT EXISTS ai_explanations (
    process_name TEXT PRIMARY KEY,
    explanation  TEXT NOT NULL,
    model        TEXT NOT NULL,
    created_ts   INTEGER NOT NULL
);
