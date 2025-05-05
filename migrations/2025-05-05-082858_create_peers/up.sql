-- Your SQL goes here
CREATE TABLE peers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    public_key TEXT NOT NULL,
    name TEXT,
    endpoint TEXT NOT NULL,
    allowed_ip TEXT NOT NULL
);
