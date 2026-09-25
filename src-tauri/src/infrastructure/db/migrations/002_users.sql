-- Users and roles for the restaurant application.
-- PINs are stored as sha256(salt + pin) to avoid plain-text storage.
-- This is adequate for a local POS on a trusted machine;
-- for internet-facing applications, use Argon2 or bcrypt.

CREATE TABLE users (
    id          INTEGER PRIMARY KEY,
    username    TEXT    NOT NULL UNIQUE,
    pin_hash    TEXT    NOT NULL,
    pin_salt    TEXT    NOT NULL,
    role        TEXT    NOT NULL CHECK (role IN ('admin', 'waiter', 'kitchen')),
    active      INTEGER NOT NULL DEFAULT 1 CHECK (active IN (0, 1)),
    created_at  TEXT    NOT NULL DEFAULT (datetime('now')),
    updated_at  TEXT    NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_users_active ON users(active);