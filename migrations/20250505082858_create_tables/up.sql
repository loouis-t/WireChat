CREATE TABLE peers (
    public_key TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    endpoint TEXT NOT NULL,
    allowed_ip TEXT NOT NULL,
    interface_ip TEXT NOT NULL
);

CREATE TABLE messages (
    sender_public_key TEXT PRIMARY KEY NOT NULL,
    receiver_public_key TEXT NOT NULL,
    message TEXT NOT NULL,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    FOREIGN KEY (sender_public_key) REFERENCES peers (public_key) ON DELETE CASCADE,
    FOREIGN KEY (receiver_public_key) REFERENCES peers (public_key) ON DELETE CASCADE
);
