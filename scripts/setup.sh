#!/usr/bin/env bash
# Verify prerequisites and install dependencies for WhatsNew development.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

echo "Checking Rust toolchain..."
if ! command -v cargo >/dev/null 2>&1; then
    echo "cargo not found. Install Rust from https://rustup.rs/ before continuing." >&2
    exit 1
fi
cargo --version
rustc --version

echo "Checking for Tauri CLI..."
if cargo install --list | grep -q '^tauri-cli '; then
    echo "tauri-cli already installed."
else
    echo "Installing tauri-cli..."
    cargo install tauri-cli --version "^2"
fi

echo "Checking Node.js..."
if ! command -v npm >/dev/null 2>&1; then
    echo "npm not found. Install Node.js 20+ from https://nodejs.org/ before continuing." >&2
    exit 1
fi
node --version
npm --version

echo "Installing frontend dependencies..."
(cd "$root/web" && npm install)

echo "Installing git hook tooling (lefthook)..."
(cd "$root" && npm install && npx lefthook install)

echo "Setup complete."
