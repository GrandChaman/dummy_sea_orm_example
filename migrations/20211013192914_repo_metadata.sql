CREATE TABLE IF NOT EXISTS repository (
    id TEXT PRIMARY KEY,
    owner TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT
);
