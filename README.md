# WhatsNew

A news aggregator. Plain and simple.

A Tauri v2 (Rust) + SvelteKit desktop app: add topics, get matched/discovered
RSS/Atom feeds, periodically refresh them, and read aggregated articles with
desktop notifications. Everything is local — SQLite for data, no backend
service.

## Stack

- **`whatsnew-core`** — Rust library crate (Tauri-free): data models, SQLite
  repositories (`sqlx`), feed fetching/parsing (`reqwest` + `feed-rs`), feed
  autodiscovery (`scraper`), topic→feed keyword matching, and retention.
- **`src-tauri`** — Tauri v2 shell: thin command adapters, app state, a
  periodic refresh scheduler, and toast notifications.
- **`web`** — SvelteKit + TypeScript + Tailwind v4 UI.

## Getting started

```powershell
./scripts/setup.ps1   # verify Rust/MSVC/Node, install tooling, npm install
./scripts/dev.ps1      # run the app in dev mode with hot reload
./scripts/test.ps1      # run the full test/lint suite (mirrors CI)
./scripts/build.ps1      # build a release installer
```

See [`CLAUDE.md`](./CLAUDE.md) for architecture details, coding conventions,
and the spec-driven development workflow used for new features.
