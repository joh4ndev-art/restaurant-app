-- Initial schema for the restaurant management application.
-- All monetary values are stored as integer cents to avoid float rounding errors.
-- Timestamps are stored as ISO-8601 text using SQLite's datetime('now').

CREATE TABLE categories (
    id          INTEGER PRIMARY KEY,
    name        TEXT    NOT NULL UNIQUE,
    sort_order  INTEGER NOT NULL DEFAULT 0,
    created_at  TEXT    NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE products (
    id            INTEGER PRIMARY KEY,
    category_id   INTEGER NOT NULL,
    name          TEXT    NOT NULL,
    description   TEXT,
    price_cents   INTEGER NOT NULL CHECK (price_cents >= 0),
    active        INTEGER NOT NULL DEFAULT 1 CHECK (active IN (0, 1)),
    created_at    TEXT    NOT NULL DEFAULT (datetime('now')),
    updated_at    TEXT    NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE RESTRICT
);

CREATE INDEX idx_products_category ON products(category_id);
CREATE INDEX idx_products_active   ON products(active);

CREATE TABLE tables (
    id          INTEGER PRIMARY KEY,
    name        TEXT    NOT NULL UNIQUE,
    zone        TEXT,
    capacity    INTEGER NOT NULL DEFAULT 4 CHECK (capacity > 0),
    status      TEXT    NOT NULL DEFAULT 'free'
                CHECK (status IN ('free', 'occupied', 'reserved', 'cleaning')),
    created_at  TEXT    NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_tables_status ON tables(status);

CREATE TABLE orders (
    id           INTEGER PRIMARY KEY,
    table_id     INTEGER,
    status       TEXT    NOT NULL DEFAULT 'open'
                 CHECK (status IN ('open', 'sent_to_kitchen', 'served', 'paid', 'cancelled')),
    total_cents  INTEGER NOT NULL DEFAULT 0 CHECK (total_cents >= 0),
    notes        TEXT,
    created_at   TEXT    NOT NULL DEFAULT (datetime('now')),
    closed_at    TEXT,
    FOREIGN KEY (table_id) REFERENCES tables(id) ON DELETE SET NULL
);

CREATE INDEX idx_orders_status ON orders(status);
CREATE INDEX idx_orders_table  ON orders(table_id);

CREATE TABLE order_items (
    id                INTEGER PRIMARY KEY,
    order_id          INTEGER NOT NULL,
    product_id        INTEGER NOT NULL,
    product_name      TEXT    NOT NULL,   -- snapshot at order time
    quantity          INTEGER NOT NULL CHECK (quantity > 0),
    unit_price_cents  INTEGER NOT NULL CHECK (unit_price_cents >= 0),
    notes             TEXT,
    status            TEXT    NOT NULL DEFAULT 'pending'
                      CHECK (status IN ('pending', 'preparing', 'ready', 'served', 'cancelled')),
    created_at        TEXT    NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (order_id)   REFERENCES orders(id)   ON DELETE CASCADE,
    FOREIGN KEY (product_id) REFERENCES products(id) ON DELETE RESTRICT
);

CREATE INDEX idx_order_items_order ON order_items(order_id);

CREATE TABLE payments (
    id            INTEGER PRIMARY KEY,
    order_id      INTEGER NOT NULL,
    method        TEXT    NOT NULL CHECK (method IN ('cash', 'card', 'mixed', 'other')),
    amount_cents  INTEGER NOT NULL CHECK (amount_cents > 0),
    created_at    TEXT    NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (order_id) REFERENCES orders(id) ON DELETE CASCADE
);

CREATE INDEX idx_payments_order ON payments(order_id);

CREATE TABLE settings (
    key         TEXT PRIMARY KEY,
    value       TEXT NOT NULL,
    updated_at  TEXT NOT NULL DEFAULT (datetime('now'))
);