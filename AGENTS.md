# AGENTS.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

Snap is a native desktop screenshot and whiteboarding app built in Rust with egui — a replacement for Microsoft Snipping Tool and Microsoft Whiteboard. Targets Windows, Linux, and macOS. Very early WIP.

## Commands

| Task | Command |
|---|---|
| Run | `cargo run` |
| Build (release) | `cargo build --release` |
| Compile check | `cargo check` |
| Lint | `cargo clippy -- -D warnings` |
| Format check | `cargo fmt --check` |
| Format | `cargo fmt` |
| Test | `cargo test` |
| Single test | `cargo test <test_name>` |

## Tech stack

- **Rust 2021 edition** with Cargo
- **egui 0.21 / eframe 0.21** — immediate-mode GUI framework (entire UI redraws every frame, no reactive state)
- **dark-light** — OS theme detection (present but not yet wired into egui)

## Architecture

**Two core traits** (defined in `main.rs`):
- `View` — renders inline into a panel via `render(&mut self, ui: &mut egui::Ui)`
- `Widget` — renders as a floating window via `name()` + `show(&mut self, ctx: &Context)`

**Layout** (assembled in `App::update`):
```
TopBottomPanel::top    (64px)  -> Header (menu bar)
TopBottomPanel::bottom (64px)  -> Footer (tool buttons + colour palette)
SidePanel::left        (64px)  -> placeholder
CentralPanel           (rest)  -> Canvas (drawing area)
```

**Key modules:**
- `canvas.rs` — freehand drawing; stores lines as `Vec<Vec<Pos2>>` in normalised 0–1 coordinates, transformed to/from screen space each frame
- `footer.rs` — bottom toolbar with tool selector and colour palette buttons
- `palette.rs` — 10 hardcoded `Color32` values accessed by index
- `center_widget.rs` — utility that measures content width and adds equal padding to horizontally centre it (needed because egui requires knowing content size upfront)
- `main_using_iced.rs` — abandoned Iced prototype, dead code, do not use

**Font:** `MesloLGM.ttf` in `assets/` is embedded at compile time via `include_bytes!` and registered as both proportional and monospace.

## Current state

- No tests, no CI, no git hooks
- No serialisation/persistence (`serde` feature flag exists on canvas structs but is not enabled in `Cargo.toml`)
- Left side panel is a placeholder

---

# Modernisation tracking

## Current status: In progress

## Team

| Agent | Role | Branch | Status |
|---|---|---|---|
| @Thinker | Research, planning, tracking | main | Active |
| @Worker1 | Phases 1-3: housekeeping, dep upgrades, verify compilation | `modernise/core-changes` | In progress |
| @Worker2 | Phases 4-5: pre-commit hooks, CI/CD workflows | `modernise/ci-hooks` | In progress |
| @Worker3 | Phase 6: cargo-dist release pipeline | — | Blocked on Phase 2 |
| @DevServers | Dev server monitoring | main | Active |
| @Reviewer | Change review, diff monitoring | main | Active |
| @QA | Phase 7: final verification, quality checks | — | Blocked on all phases |

## Phase overview

| Phase | Owner | Status |
|---|---|---|
| 1. Housekeeping | @Worker1 | In progress |
| 2. Dependency upgrade | @Worker1 | In progress |
| 3. Verify compilation | @Worker1 | Blocked on Phase 2 |
| 4. Pre-commit hooks | @Worker2 | In progress |
| 5. GitHub Actions CI | @Worker2 | In progress |
| 6. cargo-dist | @Worker3 | Blocked on Phase 2 |
| 7. Final verification | @QA | Blocked on Phases 1-6 |

## Phase 1 — Housekeeping

| Task | Status | Notes |
|---|---|---|
| 1.1 Delete `src/main_using_iced.rs` | Pending | Dead code, abandoned Iced prototype |
| 1.2 Update `.gitignore` | Pending | Add .DS_Store, Thumbs.db, *.swp, *.swo; remove Cargo.lock from ignore (binary crate) |
| 1.3 Add `rust-toolchain.toml` | Pending | Pin stable 1.93.0 with clippy + rustfmt |
| 2.1 Fix clippy: `format!("")` in footer.rs:39 | Pending | Replace with `String::new()` |
| 2.2 Fix clippy: `vec![...]` in footer.rs:30 | Pending | Replace with array literal |

## Phase 2 — Dependency upgrade

| Task | Status | Notes |
|---|---|---|
| 3.1 Upgrade egui 0.21 -> 0.33.3, eframe 0.21.3 -> 0.33.3 | Pending | |
| 3.2 Migrate `NativeOptions` in main.rs | Pending | `initial_window_size` -> `viewport: ViewportBuilder` |
| 3.3 Migrate `run_native` closure | Pending | Return `Result` — `Ok(Box::new(...))` |
| 3.4 Migrate `Frame::canvas()` in canvas.rs | Pending | Likely removed/renamed in 0.33 — needs research |
| 3.5 Fix other breaking API changes | Pending | `stroke_ui`, `Sense`, etc. |
| 3.6 Upgrade dark-light 1.0 -> 2.0 | Pending | Import only, no integration needed |
| 4.1 Add `xcap` | Pending | |
| 4.2 Add `global-hotkey` | Pending | |
| 4.3 Add `image` | Pending | |
| 4.4 Add `serde` + `serde_json` | Pending | |
| 4.5 Define serde feature flag | Pending | Fix `cfg_attr` in canvas.rs |

## Phase 3 — Verify compilation

| Task | Status | Notes |
|---|---|---|
| 5.1 `cargo fmt --check` | Pending | |
| 5.2 `cargo clippy -- -D warnings` | Pending | |
| 5.3 `cargo check` | Pending | |
| 5.4 `cargo test` | Pending | |
| 5.5 `cargo run` — app launches | Pending | |

## Phase 4 — Pre-commit hooks

| Task | Status | Notes |
|---|---|---|
| 6.1 Create `.githooks/pre-commit` | Pending | fmt + clippy |
| 6.2 Make hook executable | Pending | |
| 6.3 Document hook activation | Pending | |

## Phase 5 — GitHub Actions CI

| Task | Status | Notes |
|---|---|---|
| 7.1 `checks.yml` | Pending | fmt, clippy, check, test |
| 7.2 `build.yml` | Pending | Cross-platform build matrix (5 targets) |
| 7.3 `commit-lint.yml` | Pending | Conventional commits on PR titles |
| 7.4 `lint-workflows.yml` | Pending | actionlint on workflow changes |
| 7.5 `release-please.yml` + config | Pending | Automated release PRs |

## Phase 6 — cargo-dist

| Task | Status | Notes |
|---|---|---|
| 8.1 Init cargo-dist | Pending | |
| 8.2 Configure 5 target platforms | Pending | Windows x64, Linux x64/ARM, macOS ARM/Intel |
| 8.3 Verify workflow integrates with release-please | Pending | |
| 8.4 Test `cargo dist build` locally | Pending | |

## Phase 7 — Final verification

| Task | Status | Notes |
|---|---|---|
| 9.1 Full quality check suite | Pending | |
| 9.2 Test pre-commit hook | Pending | |
| 9.3 Review all files | Pending | |

## Issues found

| Issue | Severity | Status |
|---|---|---|
| `Cargo.lock` gitignored — wrong for binary crate | Medium | Pending fix in 1.2 |
| `Frame::canvas()` may not exist in egui 0.33 | High | Needs research during 3.4 |
| New crates will produce unused dependency warnings | Low | Accepted until feature work |
