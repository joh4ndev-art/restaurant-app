# Architecture

## Overview

Restaurant POS is a Tauri v2 desktop application with a strict layered
architecture. The goal is to keep the business logic independent of both
the framework and the database, so the app can evolve without cascading
refactors.

```
┌─────────────────────────────────────────────────────┐
│  Frontend (WebView)                                 │
│  HTML5 · CSS3 · Vanilla JS (ES modules)             │
│  Views · Store · IPC wrapper · i18n · Router        │
└──────────────────────┬──────────────────────────────┘
                       │ Tauri IPC (invoke / events)
┌──────────────────────▼──────────────────────────────┐
│  Tauri Core (Rust)                                  │
│                                                     │
│  commands/      ← IPC entry points                  │
│  services/      ← business rules                    │
│  domain/        ← entities and enums                │
│  infrastructure/                                    │
│    ├── db/          ← connection, migrations        │
│    ├── repositories/ ← SQL queries                  │
│    └── printing/     ← PDF generation               │
│                                                     │
│  state.rs       ← shared AppState (Mutex<Connection>)│
│  error.rs       ← unified AppError                  │
└──────────────────────┬──────────────────────────────┘
                       │ rusqlite
┌──────────────────────▼──────────────────────────────┐
│  SQLite (local file, WAL mode)                      │
└─────────────────────────────────────────────────────┘
```

## Layer responsibilities

| Layer | Knows about | Does NOT know about |
|---|---|---|
| **Domain** | Nothing. Pure data structures. | SQL, Tauri, HTTP, JSON. |
| **Repository** | SQL schema, `rusqlite`. | Business rules, IPC, framework. |
| **Service** | Domain + repositories. Enforces rules. | Tauri, HTTP, frontend. |
| **Commands** | Tauri, `AppState`, services. | SQL, HTML, business rules. |
| **Frontend** | Tauri IPC contract. | SQL, Rust internals. |

## Data flow — sending an order

1. User taps a product in POS (`src/js/views/pos.js`).
2. `tablesApi`/`ordersApi` (`src/js/api/ipc.js`) invokes `send_order`.
3. `commands/orders.rs` acquires the DB lock and calls `OrderService::send_to_kitchen`.
4. `OrderService` validates, opens a transaction, inserts order + items, marks the table occupied, commits.
5. Command emits `orders:changed` event.
6. All subscribed views (Kitchen, Orders, Tables) reload automatically.

## Design principles

- **Local-first.** No network, no cloud.
- **Layered separation.** Each layer has one job.
- **Minimal dependencies.** `rusqlite`, `serde`, `thiserror`, `printpdf`, `sha2`, `rand`, `tauri`.
- **Money as integer cents.** Never floats.
- **Transactions for multi-step writes.** Order + items + table status = all or nothing.
- **Snapshot of prices in order items.** Historical receipts stay accurate.
- **Events for cross-view sync.** Views don't poll; they react.

## Project structure

```
restaurant-app/
├── src/                       # Frontend
│   ├── index.html
│   ├── main.js
│   ├── js/
│   │   ├── api/ipc.js
│   │   ├── auth.js
│   │   ├── events.js
│   │   ├── router.js
│   │   ├── components/
│   │   ├── i18n/
│   │   ├── state/
│   │   ├── utils/
│   │   └── views/
│   └── styles/
│
├── src-tauri/                 # Backend
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── icons/
│   └── src/
│       ├── main.rs
│       ├── lib.rs
│       ├── error.rs
│       ├── state.rs
│       ├── commands/
│       ├── services/
│       ├── domain/
│       └── infrastructure/
│
├── docs/
├── release.sh
├── LICENSE
├── CHANGELOG.md
└── README.md
```