# Changelog

All notable changes to Restaurant POS are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Planned
- Multi-terminal support (local network).
- Direct thermal printer support (ESC/POS).
- Discounts, tips, and split bills.
- Inventory and suppliers.
- Electronic invoicing (DIAN Colombia).
- Advanced reports by waiter and by hour.

## [0.1.0] — 2026-09-29

### Added
- Initial public release.
- Table management with visual status grid.
- Point of Sale (POS) with product grid and cart.
- Order creation and status tracking.
- Kitchen Display System (KDS) with color-coded elapsed time.
- Payment processing with cash, card, and mixed methods.
- Partial payments and change calculation.
- Sales reports with filters (today, last 7 days, this month, custom).
- Top products and sales-by-day rankings.
- User management with roles (admin, waiter, kitchen).
- PIN-based login with SHA-256 hashing.
- Multi-language support (English, Spanish).
- PDF generation for sales receipts and kitchen tickets.
- Automated daily backups with manual backup and restore.
- Restaurant profile configuration.
- Local SQLite database with migrations.
- `orders:changed` events for real-time cross-view sync.
- `.deb` and `.AppImage` bundles.