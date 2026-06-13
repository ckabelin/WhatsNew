#!/usr/bin/env bash
# Build a release bundle of WhatsNew.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

cd "$root/src-tauri"
cargo tauri build

bundle_dir="$root/target/release/bundle"
echo ""
echo "Build complete. Installer(s):"
find "$bundle_dir" -type f \( -name '*.exe' -o -name '*.msi' -o -name '*.dmg' -o -name '*.AppImage' -o -name '*.deb' -o -name '*.rpm' \) 2>/dev/null | while read -r f; do
    echo "  $f"
done
