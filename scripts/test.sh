#!/usr/bin/env bash
# Run the full WhatsNew test/lint suite (mirrors CI).
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

echo "==> cargo test --workspace"
cargo test --workspace

echo "==> cargo clippy --workspace --all-targets -- -D warnings"
cargo clippy --workspace --all-targets -- -D warnings

cd "$root/web"

echo "==> npm run check"
npm run check

echo "==> npm run lint"
npm run lint

echo "All checks passed."
