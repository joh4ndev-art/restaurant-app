# Restaurant App

A native, modern, fast and secure desktop application for restaurant management:
point of sale (POS), order tickets, table management and local persistence.

Built with a **local-first** philosophy: no cloud, no external servers, no
telemetry. Everything runs on the user's machine.

---

## Table of contents

- [Features](#features)
- [Tech stack](#tech-stack)
- [Design principles](#design-principles)
- [Project structure](#project-structure)
- [Requirements](#requirements)
- [Development](#development)
- [Build](#build)
- [Documentation](#documentation)
- [Roadmap](#roadmap)
- [License](#license)

---

## Features

Planned features for the first stable release:

- Point of sale (POS) with a fast product grid
- Order tickets / kitchen commands
- Table management (free, occupied, reserved, cleaning)
- Payments (cash, card, mixed) and daily reports
- Local persistence with SQLite
- Fully offline operation

---

## Tech stack

| Layer              | Technology                                   |
| ------------------ | -------------------------------------------- |
| Core / backend     | Rust                                         |
| Desktop framework  | Tauri v2                                     |
| Frontend           | HTML5, CSS3, JavaScript (managed with pnpm)  |
| Database           | SQLite (embedded via Rust crates)            |
| Local AI assistant | Ollama + Continue (VS Code)                  |

---

## Design principles

- **Local-first:** all data lives on the user's machine.
- **Ultralight:** no heavy frameworks, no ORM, no cloud SDKs.
- **Layered:** UI ↔ IPC ↔ Services ↔ Domain ↔ Persistence.
- **Minimal dependencies:** only what is strictly necessary.
- **Secure by default:** strict CSP, minimal Tauri permissions.
- **Reusable:** the same architecture will later support clinic and
  veterinary software.

---

## Project structure

```
restaurant-app/
├── src/                       # Frontend (HTML, CSS, JS)
│   ├── index.html
│   ├── main.js
│   ├── js/
│   │   ├── api/               # IPC wrapper
│   │   ├── components/        # Reusable UI pieces
│   │   ├── views/             # Full screens
│   │   ├── state/             # Lightweight store
│   │   └── utils/             # Helpers
│   ├── styles/
│   └── assets/
│
├── src-tauri/                 # Rust backend
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   └── src/
│       ├── main.rs
│       ├── lib.rs
│       ├── state.rs           # Shared AppState (DB connection, config)
│       ├── error.rs           # Unified error type
│       ├── config.rs
│       ├── commands/          # Tauri IPC commands
│       ├── services/          # Business logic
│       ├── domain/            # Entities and rules
│       └── infrastructure/    # SQLite, migrations, repositories
│           ├── db/
│           └── repositories/
│
├── docs/                      # Technical documentation and ADRs
├── .gitignore
├── README.md
├── package.json
└── pnpm-lock.yaml
```

---

## Requirements

### Operating system

- Linux (primary target)

### System dependencies (Debian/Ubuntu)

```bash
sudo apt update
sudo apt install -y libwebkit2gtk-4.1-dev build-essential curl wget file \
  libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev
```

### System dependencies (Arch)

```bash
sudo pacman -S webkit2gtk-4.1 base-devel curl wget file openssl \
  libxdo libayatana-appindicator librsvg
```

### Toolchain

- Rust (stable)
- Node.js (LTS)
- pnpm
- Git

---

## Development

Install dependencies:

```bash
pnpm install
```

Run the app in development mode (hot reload):

```bash
pnpm tauri dev
```

---

## Build

Generate an optimized release build and Linux bundles (`.deb`, `.AppImage`):

```bash
pnpm tauri build
```

Output locations:

- Executable: `src-tauri/target/release/restaurant-app`
- Bundles: `src-tauri/target/release/bundle/`

---

## Documentation

Technical documentation lives in the `docs/` folder:

- `docs/architecture.md` — high-level architecture
- `docs/database-schema.md` — SQLite schema
- `docs/ipc-contract.md` — commands exposed to the frontend
- `docs/decisions/` — Architecture Decision Records (ADRs)

---

## Roadmap

- [x] Project scaffolding (Tauri v2 + Rust + pnpm)
- [x] Repository setup and documentation baseline
- [ ] SQLite schema and migrations
- [ ] Table management module
- [ ] POS module
- [ ] Kitchen tickets
- [ ] Payments and reports
- [ ] Future: clinic and veterinary variants

---

## License

To be defined.
