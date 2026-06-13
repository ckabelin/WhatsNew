# AGENTS.md

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
│   ├── reader.rs              # read_article()/read_url(): readable content extraction
│   ├── search.rs              # search_news(): ad-hoc Google News search, not persisted
│   └── retention.rs           # prune by retention_days / max_articles_per_topic
└── tests/{db_tests,feed_parsing,retention,matching_tests}.rs

src-tauri/
├── tauri.conf.json, capabilities/default.json, build.rs
└── src/
    ├── main.rs        # builds AppState, runs migrations, spawns scheduler
    ├── state.rs        # AppState { db: Arc<Db>, http: reqwest::Client }
    ├── scheduler.rs     # tokio interval -> refresh_topic per topic -> notify
    ├── notify.rs        # toast notification helper
    └── commands/{topics,articles,feeds,search,settings}.rs

web/
├── src/app.html, src/app.css      # Tailwind v4 + self-hosted Inter
├── src/lib/
│   ├── types.ts                   # TS mirrors of Rust models
│   ├── api/{topics,articles,feeds,search,settings}.ts  # ONLY entry points to invoke()
│   ├── stores/{topics,articles,feeds,settings,viewPreferences,search,theme}.ts  # Svelte writable stores
│   ├── relevance.ts, sort.ts       # article relevancy scoring + list sorting
│   └── components/
│       ├── ui/{Button,Input,Toggle,Modal}.svelte
│       └── {TitleBar,Sidebar,ArticleCard,ArticleListItem,ArticleHeadline,
│           ViewControls,EmptyState,TopicListItem,SearchResultItem}.svelte
└── src/routes/
    ├── +layout.svelte, +layout.ts   # TitleBar + Sidebar shell, ssr=false
    ├── +page.svelte                 # redirect to first topic or EmptyState
    ├── topic/[id]/+page.svelte      # article grid + refresh
    ├── article/[id]/+page.svelte    # in-app readable article view
    ├── search/+page.svelte          # ad-hoc news search
    ├── search/read/+page.svelte     # in-app readable view for a search result
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
./scripts/build.ps1      # cargo tauri build, prints bundle artifact paths
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

The Tauri bundle config targets all formats supported by the current OS. Local
build scripts print Windows `.exe`/`.msi`, macOS `.dmg`/`.app.tar.gz`, and Linux
`.AppImage`/`.deb`/`.rpm` artifacts when produced. Successful `main` CI runs
create run-numbered prerelease GitHub Releases with Windows, macOS Apple Silicon,
macOS Intel, and Linux binaries. Tagged releases matching `v*.*.*` publish the
same platform binaries as versioned GitHub Releases through the release workflow.

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
- **Native release builds per OS**: GitHub Releases are built on native Windows,
  macOS, and Linux runners via `tauri-apps/tauri-action`; no cross-OS desktop
  packaging is attempted locally.
- **Article list view mode + sort preference are global UI state in
  `tauri-plugin-store`** (`view-preferences.json`, via
  `web/src/lib/stores/viewPreferences.ts`), not per-topic and not in SQLite —
  consistent with the existing "structured settings in SQLite, ephemeral UI
  state in `tauri-plugin-store`" split.
- **Feed summary cleanup also resolves Google News redirect links**:
  `whatsnew_core::feeds::parse::strip_html` now re-decodes leftover tag-like
  text up to a couple more times, fixing feeds (notably Google News) that
  double-encode `<description>` HTML (`&amp;lt;a href=...&amp;gt;`) and would
  otherwise leak literal `<a href="...">...</a>` tags into summaries. When a
  feed entry's `<link>` is a `news.google.com` redirect, `resolve_link` (used
  by both `to_new_articles` and `search::to_search_article`) replaces it with
  the original article URL extracted via `extract_first_link` from the
  description's embedded `<a href>`, so the article/"read original" link
  points at the source instead of Google's redirect.
- **"Relevancy" sort has no dedicated backend score**: `web/src/lib/relevance.ts`
  scores an article client-side by how many of the topic name's significant
  words appear in its title (weighted higher) or summary. Revisit if a richer
  signal (e.g. reusing `matching::directory` keyword weights) is needed.
- **Every topic always gets a per-topic Google News RSS search feed**
  (`matching::directory::google_news_feed`, linked via
  `matching::ensure_feeds_for_topic`), in addition to any curated directory
  matches. This guarantees free-text topics (e.g. "Formula 1") surface
  relevant articles even when the curated directory has no matching category.
  `refresh_topic` calls `ensure_feeds_for_topic` to self-heal topics that
  somehow end up with zero linked feeds.
- **Topic icons are keyword-matched lucide-svelte icons**
  (`web/src/lib/topicIcon.ts`), not fetched favicons/logos - avoids any
  per-topic icon licensing concerns since lucide-svelte (ISC) is already
  bundled. The keyword rules are shared with the sidebar's auto-grouping
  feature via `web/src/lib/topicCategory.ts` (`getTopicCategory`), which adds
  a category `label` per rule; `topicIcon.ts` just delegates to it for the
  icon.
- **Sidebar width, "auto-group topics by category", expanded group state, and
  group order are global UI state in `tauri-plugin-store`**
  (`sidebarWidth`/`autoGroupTopics`/`expandedTopicGroup`/`topicGroupOrder`
  in `view-preferences.json`, via `web/src/lib/stores/viewPreferences.ts`),
  same pattern as `viewMode`/`sortMode` - not in SQLite, consistent with the
  "structured settings in SQLite, ephemeral UI state in `tauri-plugin-store`"
  split. The auto-group toggle lives on the Settings page even though it's
  not a `whatsnew_core::Settings` field. When enabled (and the topic filter
  is empty), the sidebar groups topics under collapsible, drag-reorderable
  category headers (`Sidebar.svelte`'s `groupedTopics`/`groupOrder`).
  `expandedTopicGroup` holds the single expanded category label (or `null`
  if all collapsed) - `toggleGroup` enforces accordion behavior by setting it
  to the clicked label or back to `null`, so at most one group is ever open.
  `topicGroupOrder` stores any user-dragged group order; groups not present
  in it fall back to ordering by each category's first-appearance position
  across `$topics` (`groupOrder`'s `appearanceOrder`), so new categories land
  near similarly-themed existing topics rather than at a fixed position.
  While the filter input has text, the sidebar always renders a flat
  `filteredTopics` list (grouping a filtered subset isn't meaningful), and
  reverts to the grouped/accordion view once the filter is cleared.
- **Ad-hoc "Search" page reuses the Google News RSS mechanism** (
  `whatsnew_core::search::search_news`, same `google_news_feed` builder as
  topics) but results are not persisted - no `Feed`/`Article` rows are
  created. `whatsnew_core::reader::read_url` (factored out of
  `read_article`) extracts readable content for a search result's URL
  directly, without requiring a DB-backed `Article`. The selected result is
  handed to `web/src/routes/search/read` via a small Svelte store
  (`stores/search.ts`) rather than query params, to satisfy the
  `svelte/no-navigation-without-resolve` lint rule. The search page's
  query/results/loading/error/searched state also lives in module-level
  writable stores in `stores/search.ts`
  (`searchQuery`/`searchResults`/`searchLoading`/`searchError`/
  `searchPerformed`), not local component state, so navigating away to read
  a result and back doesn't lose the search. `performSearch()` also starts
  a 60-second `setInterval` that silently re-runs `search_news` for the
  active query and replaces `searchResults` (errors are swallowed, keeping
  the last good results); the interval is module-scoped, so it keeps
  running across navigation, and is stopped by `clearSearch()` (currently
  unused, reserved for a future "clear search" action).
- **Theming is CSS-variable-based, not per-component Tailwind colors**:
  `web/src/app.css` defines semantic color tokens (`--bg`, `--surface`,
  `--text`, `--accent`, `--danger`, `--favorite`, `--highlight`,
  `--error-*`, etc.) exposed to Tailwind via `@theme inline` as utilities
  (`bg-surface`, `text-text-muted`, `border-border`, ...). Every theme/mode
  combination overrides these vars under `[data-theme="..."][data-mode="..."]`
  selectors on `<html>`. Three themes ship - `vscode` (default), `outlook`,
  `console` - each with `dark`/`light` variants (`vscode`+`dark` is the
  overall default, matching `:root`). `web/src/lib/stores/theme.ts` persists
  the chosen `themeName`/`themeMode` via `tauri-plugin-store`
  (`theme-preferences.json`), same pattern as `viewPreferences.ts`, and
  `+layout.svelte` applies them as `data-theme`/`data-mode` on
  `document.documentElement`. Components must use the semantic color
  utilities, never raw Tailwind palette colors (`neutral-*`, `blue-*`, etc.).
- **External links open in the system default browser via
  `tauri-plugin-opener`** (`@tauri-apps/plugin-opener`'s `openUrl`, gated by
  the `opener:default` capability), not in-app navigation. The source-URL
  link at the top of `routes/article/[id]/+page.svelte` and
  `routes/search/read/+page.svelte` is rendered as a `<button>` (not `<a>`)
  to avoid SvelteKit's `svelte/no-navigation-without-resolve` lint rule,
  since these are external URLs, not app routes.

## Known caveats

- `src-tauri/icons/` contains `source.png` plus the generated desktop icon set
  used by `cargo build`/`tauri build` — `tauri-build` requires `icon.ico` even
  for debug builds. Replace artwork via
  `./web/node_modules/.bin/tauri.cmd icon <logo.png>` from the repo root (see
  `src-tauri/icons/README.md`).
- `web/static/fonts/` needs `InterVariable.woff2` added manually (OFL-1.1); the
  app falls back to the system font stack without it.
- Windows toast notifications may not show in unbundled `tauri dev` — verify via
  a bundled build.
- The main window has `dragDropEnabled: false` in `tauri.conf.json`. Without
  it, WebView2's native OS file-drop handler swallows in-page HTML5
  drag-and-drop events (`dragstart`/`dragover`/`drop`), breaking the sidebar's
  topic-reordering drag handles on Windows.

## Self sustainablity

Update CLAUDE.md whenever you learn something new about the app, the architecture or basically anything that is worth persisting for future sessions.
Always keep CLAUDE.md and AGENTS.md in sync.
Update CHANGELOG.md in the same change whenever you add, change, fix, remove, or internally restructure anything notable for users, maintainers, releases, or future AI agents.

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
