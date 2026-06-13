# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.
Always keep CLAUDE.md and AGENTS.md in sync

## Project

WhatsNew — a news aggregator. Plain and simple.

A Tauri v2 (Rust) + SvelteKit desktop app: add topics, get matched/discovered RSS/Atom
feeds, periodically refresh them, and read aggregated articles with desktop
notifications. Everything is local — SQLite for data, no backend service.

## Architecture overview

Single monolith desktop app, no client/server split:

- **`whatsnew-core`** (Rust library crate, Tauri-free): models, SQLite repositories
  (via `sqlx`), feed fetching/parsing (`reqwest` + `feed-rs`), feed autodiscovery
  (`scraper`), topic→feed keyword matching against a curated directory, and
  retention/pruning. Fully testable with `cargo test -p whatsnew-core` against an
  in-memory SQLite DB — no Tauri or WebView needed.
- **`src-tauri`** (Tauri v2 shell): thin command adapters that call into
  `whatsnew-core`, app state (`AppState { db, http }`), a `tokio::time::interval`
  scheduler that periodically refreshes topics and fires toast notifications via
  `tauri-plugin-notification`, and `tauri-plugin-store` for ephemeral UI state.
- **`web`** (SvelteKit + TypeScript + Tailwind v4): the UI. Talks to the backend
  exclusively through `invoke()` wrappers in `src/lib/api/*`, which populate Svelte
  stores in `src/lib/stores/*` consumed by routes/components.

## Workspace layout

```
Cargo.toml                 # workspace root (whatsnew-core, src-tauri)
rust-toolchain.toml        # pinned stable toolchain + rustfmt/clippy

crates/whatsnew-core/
├── migrations/0001_init.sql   # topics, feeds, topic_feeds, articles, settings
├── assets/feed_directory.json # curated feed directory for keyword matching
├── src/
│   ├── lib.rs, error.rs, models.rs
│   ├── db/{mod,topics,feeds,articles,settings}.rs   # sqlx repositories
│   ├── feeds/{mod,fetch,parse,discovery}.rs         # fetch, feed-rs parsing, autodiscovery
│   ├── matching/{mod,directory}.rs                  # keyword matcher over feed_directory.json
│   ├── refresh.rs            # refresh_topic(): fetch + parse + store + retention
│   └── retention.rs           # prune by retention_days / max_articles_per_topic
└── tests/{db_tests,feed_parsing,retention}.rs

src-tauri/
├── tauri.conf.json, capabilities/default.json, build.rs
└── src/
    ├── main.rs        # builds AppState, runs migrations, spawns scheduler
    ├── state.rs        # AppState { db: Arc<Db>, http: reqwest::Client }
    ├── scheduler.rs     # tokio interval -> refresh_topic per topic -> notify
    ├── notify.rs        # toast notification helper
    └── commands/{topics,articles,feeds,settings}.rs

web/
├── src/app.html, src/app.css      # Tailwind v4 + self-hosted Inter
├── src/lib/
│   ├── types.ts                   # TS mirrors of Rust models
│   ├── api/{topics,articles,feeds,settings}.ts     # ONLY entry points to invoke()
│   ├── stores/{topics,articles,settings}.ts        # Svelte writable stores
│   └── components/
│       ├── ui/{Button,Input,Toggle,Modal}.svelte
│       └── {TitleBar,Sidebar,ArticleCard,EmptyState,TopicListItem}.svelte
└── src/routes/
    ├── +layout.svelte, +layout.ts   # TitleBar + Sidebar shell, ssr=false
    ├── +page.svelte                 # redirect to first topic or EmptyState
    ├── topic/[id]/+page.svelte      # article grid + refresh
    ├── topics/+page.svelte          # add/remove topics, notification toggles
    └── settings/+page.svelte        # retention/cache/refresh settings form

scripts/{setup,dev,build,test}.ps1   # PowerShell dev workflow (see Commands below)
scripts/{setup,dev,build,test}.sh     # bash equivalents (macOS/Linux/WSL)
.github/workflows/{ci,release}.yml
```

## Commands

```powershell
./scripts/setup.ps1   # verify Rust/MSVC/Node, install tauri-cli, npm install
./scripts/dev.ps1      # npm run tauri dev (hot reload)
./scripts/test.ps1      # cargo test --workspace, clippy, npm run check, npm run lint
./scripts/build.ps1      # npm run tauri build, prints installer path
```

Bash equivalents (macOS/Linux/WSL) live alongside them as `scripts/*.sh` and run the
same steps (the setup script skips the MSVC linker check, which is Windows-only).

Equivalent raw commands: `cargo test -p whatsnew-core`, `cargo test --workspace`,
`cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all --check`;
in `web/`: `npm run check`, `npm run lint`, `npm run build`; in `src-tauri/`:
`cargo tauri dev`, `cargo tauri build` (the Tauri CLI must be invoked from
`src-tauri/` since that's where `tauri.conf.json` lives; its
`beforeDevCommand`/`beforeBuildCommand` shell out to `npm --prefix web run ...`
from the repo root).

## Coding conventions

- `whatsnew-core` stays Tauri-free and platform-agnostic — no `tauri` dependency,
  no Windows-specific code. Tauri commands in `src-tauri/src/commands/*` are thin:
  deserialize args, call a core function, map `whatsnew_core::Error` to a
  serializable error.
- Errors in `whatsnew-core` use `thiserror` (`error.rs`).
- SQL migrations in `crates/whatsnew-core/migrations/` are **append-only** — never
  edit a merged migration; add a new numbered file instead.
- The frontend never calls raw `invoke()` outside `web/src/lib/api/*`. Components
  read/write Svelte stores in `web/src/lib/stores/*`, which call the `api/*`
  wrappers.
- Rust: `rustfmt` + `cargo clippy --workspace --all-targets -- -D warnings`.
  Frontend: Prettier + ESLint + `svelte-check`. Both enforced in CI.

## Adding a new Tauri command

1. Add the core logic/function to `whatsnew-core` (with tests) if it doesn't
   already exist there.
2. Add a thin `#[tauri::command]` wrapper in the relevant
   `src-tauri/src/commands/*.rs` file that pulls `AppState` and calls into
   `whatsnew-core`.
3. Register it in the `tauri::generate_handler![...]` list in `src-tauri/src/main.rs`.
4. Add an `invoke()` wrapper in `web/src/lib/api/<domain>.ts`.
5. Call it from a store action in `web/src/lib/stores/<domain>.ts`, and use the
   store from components/routes.
6. Update `web/src/lib/types.ts` if the Rust model changed shape.

## Key decisions log

- **`feed-rs`** over `rss`/`atom_syndication`: single unified RSS/Atom/JSON-Feed
  model.
- **`sqlx` direct, not `tauri-plugin-sql`**: `whatsnew-core` owns its schema and
  migrations (`sqlx::migrate!()`) and is testable with in-memory SQLite with zero
  Tauri involvement.
- **No cron plugin**: periodic refresh is a plain `tokio::time::interval` task in
  `src-tauri/src/scheduler.rs` calling `whatsnew_core::refresh::refresh_topic()`.
- **`reqwest` with `rustls-tls`**, not OpenSSL/native-tls — avoids OpenSSL build/
  licensing concerns.
- **Structured settings live in SQLite** (`settings` table via `whatsnew-core`);
  `tauri-plugin-store` is only for ephemeral UI/window state.
- **Curated feed directory + autodiscovery + manual entry** for MVP feed
  discovery — no external search-engine scraping.

## Known caveats

- `src-tauri/icons/` contains a generated placeholder icon set (from
  `source-placeholder.png`) so `cargo build`/`tauri build` succeed —
  `tauri-build` requires `icon.ico` even for debug builds. Replace with real
  artwork via `./web/node_modules/.bin/tauri.cmd icon <logo.png>` from the repo
  root (see `src-tauri/icons/README.md`).
- `web/static/fonts/` needs `InterVariable.woff2` added manually (OFL-1.1); the
  app falls back to the system font stack without it.
- Windows toast notifications may not show in unbundled `tauri dev` — verify via
  a bundled build.

## Self sustainablity

Update CLAUDE.md whenever you learn something new about the app, the architecture or basically anything that is worth persisting for future sessions.

## Spec-Driven Development workflow

Features are built through the Speckit skills, in this order:

1. `/speckit.constitution` — establish/update project principles in `.specify/memory/constitution.md`.
2. `/speckit.specify` — write a feature spec.
3. `/speckit.clarify` — resolve ambiguities in the spec before planning.
4. `/speckit.plan` — generate the implementation plan for a spec.
5. `/speckit.tasks` — break the plan into actionable tasks.
6. `/speckit.analyze` — cross-check spec/plan/tasks for consistency.
7. `/speckit.implement` — execute the tasks.
8. `/speckit.checklist` — generate quality checklists for a feature.

Each feature's generated plan (`specs/<feature>/plan.md`) becomes the authoritative
source for that feature's tech stack, build/run/test commands, and architecture.
The foundation + MVP described above corresponds to `specs/001-whatsnew-mvp/` going
forward — start there for the next feature's context.

<!-- SPECKIT START -->
For additional context about technologies to be used, project structure,
shell commands, and other important information, read the current plan
<!-- SPECKIT END -->
