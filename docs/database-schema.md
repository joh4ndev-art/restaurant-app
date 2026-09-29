# Database Schema

SQLite, WAL mode, embedded in the app binary. Migrations are applied
automatically on startup.

## Migrations

| Version | Name | Description |
|---|---|---|
| 1 | `initial_schema` | Categories, products, tables, orders, order items, payments, settings |
| 2 | `users_and_roles` | Users with hashed PINs and roles |

Migration history is stored in `schema_migrations`.

## Tables

### `categories`

| Column | Type | Notes |
|---|---|---|
| id | INTEGER PK | |
| name | TEXT | UNIQUE |
| sort_order | INTEGER | Controls display order |
| created_at | TEXT | ISO 8601 |

### `products`

| Column | Type | Notes |
|---|---|---|
| id | INTEGER PK | |
| category_id | INTEGER FK | → `categories.id`, `ON DELETE RESTRICT` |
| name | TEXT | |
| description | TEXT | nullable |
| price_cents | INTEGER | Price in cents. Never floats. |
| active | INTEGER | 0 or 1 |
| created_at | TEXT | |
| updated_at | TEXT | |

### `tables`

| Column | Type | Notes |
|---|---|---|
| id | INTEGER PK | |
| name | TEXT | UNIQUE |
| zone | TEXT | nullable |
| capacity | INTEGER | > 0 |
| status | TEXT | `free` \| `occupied` \| `reserved` \| `cleaning` |
| created_at | TEXT | |

### `orders`

| Column | Type | Notes |
|---|---|---|
| id | INTEGER PK | |
| table_id | INTEGER FK | → `tables.id`, `ON DELETE SET NULL`, nullable |
| status | TEXT | `open` \| `sent_to_kitchen` \| `served` \| `paid` \| `cancelled` |
| total_cents | INTEGER | |
| notes | TEXT | nullable |
| created_at | TEXT | |
| closed_at | TEXT | nullable |

### `order_items`

| Column | Type | Notes |
|---|---|---|
| id | INTEGER PK | |
| order_id | INTEGER FK | → `orders.id`, `ON DELETE CASCADE` |
| product_id | INTEGER FK | → `products.id`, `ON DELETE RESTRICT` |
| product_name | TEXT | **Snapshot** at order time |
| quantity | INTEGER | > 0 |
| unit_price_cents | INTEGER | **Snapshot** at order time |
| notes | TEXT | nullable |
| status | TEXT | `pending` \| `preparing` \| `ready` \| `served` \| `cancelled` |
| created_at | TEXT | |

### `payments`

| Column | Type | Notes |
|---|---|---|
| id | INTEGER PK | |
| order_id | INTEGER FK | → `orders.id`, `ON DELETE CASCADE` |
| method | TEXT | `cash` \| `card` \| `mixed` \| `other` |
| amount_cents | INTEGER | > 0 |
| created_at | TEXT | |

### `settings`

| Column | Type | Notes |
|---|---|---|
| key | TEXT PK | |
| value | TEXT | |
| updated_at | TEXT | |

Common keys: `locale`, `restaurant_name`, `restaurant_address`,
`restaurant_phone`, `restaurant_footer`, `receipt_folder`,
`backup_folder`, `auto_backup_enabled`, `last_auto_backup_date`.

### `users`

| Column | Type | Notes |
|---|---|---|
| id | INTEGER PK | |
| username | TEXT | UNIQUE |
| pin_hash | TEXT | SHA-256 of `salt + pin` |
| pin_salt | TEXT | Random 16-byte hex |
| role | TEXT | `admin` \| `waiter` \| `kitchen` |
| active | INTEGER | 0 or 1 |
| created_at | TEXT | |
| updated_at | TEXT | |

### `schema_migrations`

| Column | Type | Notes |
|---|---|---|
| version | INTEGER PK | |
| name | TEXT | |
| applied_at | TEXT | |

## Design decisions

- **Money in cents.** Prevents floating-point rounding errors.
- **Timestamps as ISO 8601 text.** SQLite native, sortable, comparable.
- **Snapshot in `order_items`.** Historical receipts stay accurate even
  after product renames or price changes.
- **`ON DELETE RESTRICT` on categories.** Cannot delete a category with products.
- **`ON DELETE SET NULL` on orders.table_id.** Tables can be deleted
  without losing order history.
- **WAL mode.** Better concurrency, less locking.
- **Foreign keys enabled per connection.** SQLite disables them by default.