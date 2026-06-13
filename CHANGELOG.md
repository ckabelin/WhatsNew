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
- Added a self-contained article reader page that fetches and extracts readable
  article text inside the app.
- Added real WhatsNew app icon artwork and regenerated the Tauri desktop icon
  set.
- Added persistent topic ordering with drag-and-drop and up/down controls in the
  sidebar.
- Show article publish times alongside publish dates in article cards and the
  in-app reader.
- Added a system tray icon with show and quit actions; closing the window now
  hides WhatsNew to the tray instead of exiting.
- Added a popular topic cloud below the topic input for quick topic preselection.
- Kept the article reader back button sticky so it remains available at any
  scroll position.
- Show non-ad article images in the in-app reader when they can be extracted from
  the source article page.
- Show the current article title in the sticky reader toolbar.
- Preserve article image placement relative to text in the in-app reader when
  extracting readable content.
- Added native Windows, macOS, and Linux desktop build coverage and GitHub
  Release binaries for tagged releases.

### Internal

- Changed the release workflow so pushes to `main` create run-numbered
  prerelease GitHub Releases with desktop binaries, while `v*.*.*` tags remain
  versioned releases.
- Documented in `AGENTS.md` and `CLAUDE.md` that notable changes must update
  `CHANGELOG.md` in the same change.
- Expanded the Tauri bundle configuration and CI/release workflows to validate
  desktop builds across Windows, macOS, and Linux.
- Made CI desktop builds wait for both Rust and frontend checks to pass before
  running.
- Fixed CI desktop builds to use the npm-installed Tauri CLI instead of assuming
  `cargo tauri` is preinstalled on GitHub runners.

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
