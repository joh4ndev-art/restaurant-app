#!/usr/bin/env bash
set -euo pipefail

echo "▸ Building Restaurant POS..."
cd "$(dirname "$0")"
pnpm tauri build
echo ""
echo "▸ Artifacts:"
ls -lh src-tauri/target/release/bundle/deb/*.deb 2>/dev/null || true
ls -lh src-tauri/target/release/bundle/appimage/*.AppImage 2>/dev/null || true
echo ""
echo "✓ Done."
