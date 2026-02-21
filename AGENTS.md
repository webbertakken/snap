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

- **Rust 2021 edition** with Cargo (pinned via `rust-toolchain.toml`)
- **egui 0.33 / eframe 0.33** — immediate-mode GUI framework (entire UI redraws every frame, no reactive state)
- **dark-light 2.0** — OS theme detection (present but not yet wired into egui)
- **xcap** — screen capture
- **global-hotkey** — global keyboard shortcuts
- **image** — image processing
- **serde / serde_json** — serialisation (behind `serde` feature flag)

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

**Font:** `MesloLGM.ttf` in `assets/` is embedded at compile time via `include_bytes!` and registered as both proportional and monospace.

## Current state

- CI: GitHub Actions (checks, cross-platform build, commit lint, workflow lint, release-please, cargo-dist)
- Pre-commit hooks: fmt + clippy (`.githooks/pre-commit`, activate with `git config core.hooksPath .githooks`)
- cargo-dist: automated cross-platform release builds for 5 targets (Windows x64, Linux x64/ARM64, macOS ARM64/Intel)
- Serialisation behind `serde` feature flag (not enabled by default)
- Left side panel is a placeholder
