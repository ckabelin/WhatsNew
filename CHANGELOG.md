# Changelog

All notable changes to WhatsNew are recorded here.

This file is maintained by both human contributors and AI coding agents. Add
entries as part of the same change that introduces user-visible behavior,
architecture decisions, migrations, dependency changes, or operational changes.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project uses calendar-dated releases until a versioning policy is
introduced.

## Maintenance Rules

- Keep the newest entries at the top.
- Add work-in-progress entries under `Unreleased`.
- Prefer concise, user- or maintainer-facing descriptions over commit-message
  detail.
- Group entries under `Added`, `Changed`, `Fixed`, `Removed`, `Security`, or
  `Internal`.
- Link pull requests, issues, specs, or ADRs when they exist.
- Move `Unreleased` entries into a dated release section when cutting a release.
- Do not delete historical entries unless correcting an error.

## Unreleased

### Added

- Created this actively maintained changelog for project history and release
  notes.

### Internal

- Documented in `AGENTS.md` and `CLAUDE.md` that notable changes must update
  `CHANGELOG.md` in the same change.

## 2026-06-13

### Added

- Implemented the WhatsNew foundation and MVP desktop app:
  topic management, RSS/Atom/JSON Feed matching and refresh, article browsing,
  retention settings, background refresh, and desktop notification support.
- Added `whatsnew-core`, a Tauri-free Rust crate for models, SQLite
  repositories, feed fetching/parsing, feed discovery, topic/feed matching,
  refresh orchestration, and retention pruning.
- Added the Tauri v2 desktop shell with command adapters, app state, scheduler,
  notifications, and local SQLite migrations.
- Added the SvelteKit frontend with API wrappers, stores, app shell, topic
  screens, article list, and settings screen.
- Added PowerShell and bash setup, dev, test, and build scripts.
- Added GitHub Actions workflows for CI and draft releases.

### Internal

- Documented project architecture, commands, conventions, and known caveats in
  `README.md`, `CLAUDE.md`, and `AGENTS.md`.
