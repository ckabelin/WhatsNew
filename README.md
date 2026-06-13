# WhatsNew

WhatsNew is a local-first desktop news aggregator.

Add topics, let the app link matching RSS/Atom feeds from its bundled directory,
refresh them periodically, and read the resulting article stream in a Tauri
desktop app. Data stays on the machine in SQLite; there is no hosted backend
service.

## Current Status

This repository contains the foundation and MVP implementation:

- Topic management: create, rename, delete, and toggle per-topic notifications.
- Feed matching and refresh: topic creation links matching feeds from the curated
  directory, then refresh fetches those feeds, parses RSS/Atom/JSON Feed entries,
  stores new articles, and tracks feed cache/error state.
- Article reading: browse articles by topic and open article links externally.
- Settings: retention days, max articles per topic, max cache size setting,
  refresh interval, and global notifications toggle.
- Background refresh: a Tauri task periodically refreshes every topic and prunes
  old articles according to settings.
- Feed discovery support: the backend can discover feed links from a website's
  alternate-link metadata and add/remove feeds for a topic. The visible MVP UI is
  still centered on topic and article workflows.

## Tech Stack

- Rust workspace with a Tauri v2 desktop shell.
- `whatsnew-core`, a Tauri-free Rust library for models, SQLite repositories,
  feed fetching/parsing, feed discovery, topic/feed matching, refresh, and
  retention.
- SQLite through `sqlx`, with migrations owned by `whatsnew-core`.
- `reqwest` with `rustls-tls` for HTTP.
- `feed-rs` for RSS, Atom, and JSON Feed parsing.
- SvelteKit, Svelte 5, TypeScript, Tailwind CSS v4, and lucide-svelte for the UI.
- GitHub Actions CI for Rust checks, frontend checks, and desktop compile
  checks on Windows, macOS, and Linux.

## Repository Layout

```text
.
|-- Cargo.toml                  # Rust workspace: whatsnew-core + src-tauri
|-- rust-toolchain.toml         # pinned stable toolchain with rustfmt/clippy
|-- package.json                # root tooling, currently lefthook
|-- scripts/                    # setup/dev/test/build wrappers
|-- crates/
|   `-- whatsnew-core/
|       |-- migrations/         # SQLite schema migrations
|       |-- assets/             # curated feed directory
|       |-- src/                # core app logic
|       `-- tests/              # core integration tests
|-- src-tauri/
|   |-- tauri.conf.json         # Tauri app and bundle config
|   |-- capabilities/           # Tauri command permissions
|   |-- icons/                  # app icon source and generated desktop icon set
|   `-- src/                    # app state, scheduler, notifications, commands
|-- web/
|   |-- package.json            # SvelteKit frontend scripts/dependencies
|   |-- static/                 # static assets and optional fonts
|   `-- src/
|       |-- lib/api/            # only frontend entry points to invoke()
|       |-- lib/stores/         # Svelte stores that call api wrappers
|       |-- lib/components/     # app and UI components
|       `-- routes/             # app screens
`-- .github/workflows/          # CI and draft release workflows
```

## Prerequisites

- Rust via rustup.
- Node.js 20 or newer.
- npm.
- Tauri v2 prerequisites for your operating system.
- On Windows, MSVC Build Tools with the "Desktop development with C++" workload.
- On Linux, WebKitGTK/Tauri system libraries. CI installs the required Ubuntu
  packages in `.github/workflows/ci.yml`.

## Setup

From the repository root:

```powershell
./scripts/setup.ps1
```

The setup script verifies Rust and Node, installs `tauri-cli` if needed, installs
frontend dependencies under `web/`, and installs root git hook tooling.

Bash equivalents are available for macOS/Linux/WSL:

```bash
./scripts/setup.sh
```

## Development

Run the desktop app in development mode:

```powershell
./scripts/dev.ps1
```

The script runs `cargo tauri dev` from `src-tauri/`. Tauri then starts the
SvelteKit dev server through `beforeDevCommand` and loads it from
`http://localhost:5173`.

Equivalent manual command:

```powershell
cd src-tauri
cargo tauri dev
```

## Testing and Checks

Run the full local check suite:

```powershell
./scripts/test.ps1
```

The script runs:

```text
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
npm run check      # in web/
npm run lint       # in web/
```

Useful focused commands:

```powershell
cargo test -p whatsnew-core
cargo fmt --all --check
cd web
npm run check
npm run lint
npm run build
```

CI runs Rust checks on Windows and Ubuntu, frontend lint/check/build on Ubuntu,
and a `cargo tauri build --no-bundle --ci` desktop compile check on Windows,
macOS, and Linux.

## Build

Create a release bundle for the current operating system:

```powershell
./scripts/build.ps1
```

The Tauri config targets all bundle formats supported by the current operating
system. Local builds print bundle paths from `target/release/bundle`, including
Windows `.exe`/`.msi`, macOS `.dmg`/`.app.tar.gz`, and Linux `.AppImage`,
`.deb`, or `.rpm` artifacts when those formats are produced.

Manual command:

```powershell
cd src-tauri
cargo tauri build
```

Tagged releases matching `v*.*.*` trigger the GitHub Actions release workflow.
The workflow builds native Windows, macOS Apple Silicon, macOS Intel, and Linux
bundles, then uploads the installers to the GitHub Release for that tag through
`tauri-apps/tauri-action`.

## Architecture Notes

`whatsnew-core` is deliberately free of Tauri and platform-specific code. It owns
the schema, repository functions, feed parsing, feed discovery, matching, refresh,
and retention logic. Core tests use SQLite directly and do not need a WebView.

`src-tauri` is a thin shell. It opens the app data SQLite database, manages shared
state, registers command adapters, starts the background scheduler, and sends
desktop notifications through `tauri-plugin-notification`.

`web` talks to Rust only through `invoke()` wrappers in `web/src/lib/api/*`.
Components and routes should use stores from `web/src/lib/stores/*` instead of
calling Tauri commands directly.

## App Data

At startup, the Tauri app resolves the platform app data directory, creates it if
needed, and opens:

```text
whatsnew.db
```

The database stores topics, feeds, topic/feed links, articles, and settings.

## Known Caveats

- `src-tauri/icons/` contains `source.png` and the generated desktop icon set used
  by Tauri builds.
- `web/static/fonts/` does not include `InterVariable.woff2` by default. The app
  falls back to the system font stack unless the font is added manually.
- Windows toast notifications may not appear reliably in unbundled `tauri dev`;
  verify notification behavior in a bundled build.
- SQL migrations are append-only once merged. Add a new numbered migration rather
  than editing an existing released migration.

## Contributor Notes

Detailed architecture guidance, coding conventions, and the project workflow live
in `CLAUDE.md` and `AGENTS.md`. Keep those two files in sync when updating agent
or contributor instructions.

The codebase expects:

- Rust formatted with `rustfmt` and clean under clippy with warnings denied.
- Frontend formatted with Prettier and clean under ESLint and `svelte-check`.
- New Tauri commands to flow through core logic, Tauri command wrappers,
  frontend API wrappers, stores, and then components/routes.

## License

MIT. See `LICENSE`.
