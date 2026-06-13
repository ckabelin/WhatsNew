#!/usr/bin/env bash
# Run WhatsNew in development mode with hot reload.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

cd "$root/src-tauri"
cargo tauri dev
