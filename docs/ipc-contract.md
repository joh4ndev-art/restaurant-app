# IPC Contract

All communication between the frontend and the Rust backend uses Tauri's
`invoke` for request/response and Tauri's events for push notifications.

## Response shape

Every command returns one of:

```json
{ "ok": true,  "data": <payload> }
```

```json
{ "ok": false, "error": { "code": "...", "message": "..." } }
```

The frontend's `call()` helper in `src/js/api/ipc.js` normalizes this.

Error codes: `database_error`, `migration_error`, `not_found`,
`validation_error`, `internal_error`.

## Commands

### Tables

| Command | Arguments | Returns |
|---|---|---|
| `list_tables` | — | `Table[]` |
| `create_table` | `{ input: { name, zone?, capacity } }` | `Table` |
| `update_table_status` | `{ id, status }` | `Table` |
| `delete_table` | `{ id }` | `()` |

### Categories

| Command | Arguments | Returns |
|---|---|---|
| `list_categories` | — | `Category[]` |
| `create_category` | `{ input: { name } }` | `Category` |
| `delete_category` | `{ id }` | `()` |

### Products

| Command | Arguments | Returns |
|---|---|---|
| `list_products` | — | `Product[]` |
| `create_product` | `{ input: { category_id, name, description?, price_cents } }` | `Product` |
| `update_product` | `{ input: { id, category_id, name, description?, price_cents, active } }` | `Product` |
| `delete_product` | `{ id }` | `()` |

### Orders

| Command | Arguments | Returns |
|---|---|---|
| `send_order` | `{ input: { table_id, notes?, items: [{ product_id, quantity, notes? }] } }` | `Order` |
| `list_open_orders` | — | `Order[]` |
| `get_order` | `{ id }` | `Order` |
| `cancel_order` | `{ id }` | `()` |
| `get_active_order_for_table` | `{ table_id }` | `Order \| null` |
| `mark_order_served` | `{ id }` | `Order` |
| `list_orders_by_status` | `{ statuses: string[] }` | `Order[]` |

**Events emitted:** `send_order`, `cancel_order`, `mark_order_served`,
and `pay_order` emit `orders:changed` after success.

### Payments

| Command | Arguments | Returns |
|---|---|---|
| `pay_order` | `{ input: { order_id, method, amount_cents } }` | `PaymentSummary` |
| `get_payment_summary` | `{ order_id }` | `PaymentSummary` |

### Users

| Command | Arguments | Returns |
|---|---|---|
| `has_users` | — | `bool` |
| `login` | `{ input: { username, pin } }` | `User` |
| `list_users` | — | `User[]` |
| `create_user` | `{ input: { username, pin, role } }` | `User` |
| `update_user` | `{ input: { id, username, role, active } }` | `User` |
| `change_user_pin` | `{ input: { id, pin } }` | `()` |
| `delete_user` | `{ id }` | `()` |

### Settings

| Command | Arguments | Returns |
|---|---|---|
| `get_setting` | `{ key }` | `string \| null` |
| `set_setting` | `{ key, value }` | `()` |

### Printing

| Command | Arguments | Returns |
|---|---|---|
| `get_restaurant_info` | — | `RestaurantInfo` |
| `save_restaurant_info` | `{ info }` | `()` |
| `get_receipt_folder` | — | `string` |
| `set_receipt_folder` | `{ path }` | `()` |
| `print_receipt` | `{ order_id }` | `PrintResult` |
| `print_kitchen_ticket` | `{ order_id }` | `PrintResult` |
| `open_file` | `{ path }` | `()` |

### Reports

| Command | Arguments | Returns |
|---|---|---|
| `generate_report` | `{ input: { from, to } }` | `ReportsBundle` |

### Backups

| Command | Arguments | Returns |
|---|---|---|
| `list_backups` | — | `BackupInfo[]` |
| `create_backup` | — | `BackupInfo` |
| `restore_backup` | `{ filename }` | `()` |
| `delete_backup` | `{ filename }` | `()` |
| `get_backup_folder` | — | `string` |
| `set_backup_folder` | `{ path }` | `()` |
| `get_auto_backup` | — | `bool` |
| `set_auto_backup` | `{ enabled }` | `()` |

## Events

| Event | Payload | Emitted by |
|---|---|---|
| `orders:changed` | `()` | Order and payment commands |

Frontend listens once in `src/js/events.js` and dispatches to all
subscribers.