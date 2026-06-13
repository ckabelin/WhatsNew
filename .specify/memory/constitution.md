# WhatsNew Constitution

## Core Principles

### I. Monolith Desktop Architecture
WhatsNew is a single Tauri v2 desktop application. There is no client/server split and no
external backend service; all persistence is local (SQLite database + app-data files in the
OS app-data directory). All network I/O (feed fetching, autodiscovery) happens on the Rust
side via `reqwest`; the frontend never performs cross-origin `fetch()` calls to remote hosts.
This keeps CORS out of the picture entirely and centralizes timeout, retry, and error handling
in one place.

### II. Open Source Only
Every dependency — Rust crates, npm packages, fonts, icons, and seed feed sources — must be
permissively licensed (MIT, Apache-2.0, BSD-2/3-Clause, ISC, or CC with redistribution
allowed). GPL/LGPL/AGPL runtime dependencies are disallowed unless no viable alternative
exists, and any such exception must be explicitly documented and justified in the relevant
`specs/*/research.md`. New major dependencies should have their license noted in the
implementation plan or research doc that introduces them.

### III. Core Logic Independently Testable
Business logic — data models, the SQLite repository layer, feed fetching/parsing, feed
discovery, topic-to-feed matching, and retention/pruning — lives in the `whatsnew-core`
library crate, which has zero dependency on Tauri or any UI toolkit. `whatsnew-core` must be
fully testable via `cargo test -p whatsnew-core` without launching the app shell or a
WebView. Tauri commands in `src-tauri` are thin adapters that call into `whatsnew-core` and
translate results to/from the frontend.

### IV. Test-First for Core Logic
New functionality in `whatsnew-core` (parsers, the topic/feed matcher, database migrations,
retention rules) is accompanied by unit or integration tests written alongside or before the
implementation. UI and Tauri-shell glue code is validated primarily through manual QA and
light smoke tests for the MVP; full end-to-end UI automation is a future enhancement, not an
MVP requirement.

### V. Windows-First, Cross-Platform-Ready
The primary build and release target is `x86_64-pc-windows-msvc`, and the UI is designed to
feel native on Windows. `whatsnew-core` must remain platform-agnostic — no
Windows-specific code paths. Any platform-specific behavior (notifications, file-system
paths, window chrome) is isolated behind Tauri plugins or thin shell-layer code so that
macOS/Linux builds can be added later with minimal rework.

### VI. Style & Tooling
Rust code is formatted with `rustfmt` and must pass `cargo clippy --workspace --all-targets --
-D warnings`. Frontend code (TypeScript/Svelte) is formatted with Prettier and linted with
ESLint, and must pass `svelte-check`. Both are enforced in CI and are expected to pass before
merging to `main`. SQL migrations in `whatsnew-core/migrations` are append-only and are never
edited after being merged.

## Technology Stack

The following stack decisions are locked for the project and should not be re-litigated
without an explicit amendment to this document:

- **App shell**: Tauri v2 (Rust backend, WebView2 on Windows).
- **Frontend**: SvelteKit + TypeScript, Tailwind CSS v4, `lucide-svelte` icons, self-hosted
  Inter font (OFL-1.1).
- **Feed parsing**: `feed-rs` (unified RSS/Atom/JSON Feed model).
- **HTML parsing (feed autodiscovery)**: `scraper`.
- **HTTP client**: `reqwest` with `rustls-tls` (no OpenSSL/native-tls).
- **Database**: SQLite via `sqlx` (sqlite, `runtime-tokio-rustls`, bundled libsqlite3),
  owned and migrated entirely by `whatsnew-core` via `sqlx::migrate!()`. The
  `tauri-plugin-sql` plugin is intentionally not used.
- **Settings storage**: structured settings (retention, notification toggles, refresh
  interval) live in the SQLite `settings` table via `whatsnew-core`. `tauri-plugin-store`
  is used only for ephemeral UI/window state.
- **Notifications**: `tauri-plugin-notification` for Windows toast notifications.
- **Scheduling**: a `tokio::time::interval` task in the Tauri shell drives periodic feed
  refresh by calling a core-crate `refresh_topic()` function. No dedicated cron plugin.

## Development Workflow

All feature work follows the Speckit SDD workflow described in `CLAUDE.md`:
`/speckit.constitution` → `/speckit.specify` → `/speckit.clarify` → `/speckit.plan` →
`/speckit.tasks` → `/speckit.analyze` → `/speckit.implement` → `/speckit.checklist`. Each
feature's `specs/<feature>/plan.md` is the authoritative source for that feature's technical
decisions, within the bounds set by this constitution.

CI must pass `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
`cargo test --workspace`, `npm run lint`, `npm run check`, and `npm run build` before a PR can
merge to `main`. Releases are tag-triggered: pushing a `v*.*.*` tag builds and publishes a
Windows installer to a draft GitHub Release.

## Governance

This constitution supersedes ad hoc technical decisions. Amendments are made via a PR that
updates this file and bumps the version below. Any deviation from a stated principle (e.g.
introducing a GPL dependency, or coupling `whatsnew-core` to Tauri) must be called out
explicitly in the relevant plan's "Complexity Tracking" section with a justification.

**Version**: 1.0.0 | **Ratified**: 2026-06-13 | **Last Amended**: 2026-06-13
