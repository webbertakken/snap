## Context

Snap is a very early-stage native desktop screenshot and whiteboarding app built in Rust with egui. The codebase has 7 source files (~300 lines total), no tests, no CI, and dependencies that are 12 minor versions behind. The project needs a solid foundation before feature work can begin.

Current state:
- egui 0.21.0 / eframe 0.21.3 (latest is 0.33.3)
- dark-light 1.0.0 (latest is 2.0.0)
- No rust-toolchain.toml, no CI, no pre-commit hooks, no .gitignore
- Dead code (main_using_iced.rs), clippy errors, orphaned serde cfg_attr
- No crates for core functionality (screenshot capture, hotkeys, image handling, persistence)

The user's other project (streamer, a Tauri app) has a mature CI/CD pipeline that serves as a reference pattern.

## Goals / Non-Goals

**Goals:**
- All dependencies at latest versions, compiling cleanly with zero warnings
- New crates added and available for feature development (xcap, global-hotkey, image, serde)
- CI pipeline running on every PR and push to main (fmt, clippy, check, test)
- Cross-platform build matrix (Windows, Linux x64/ARM, macOS ARM/Intel) via cargo-dist
- Automated releases via release-please + cargo-dist
- Pre-commit hooks enforcing quality locally
- Clean codebase: no dead code, no clippy errors, no orphaned config

**Non-Goals:**
- Implementing screenshot capture, hotkey registration, or persistence features (crates are added but not wired up)
- Writing application tests (no meaningful logic to test yet; CI will run `cargo test` which passes with zero tests)
- Changing application architecture or UI layout
- Adding new UI features or modifying existing drawing behaviour
- Code signing or notarisation for release binaries

## Decisions

### 1. egui migration strategy: direct upgrade to 0.33.3
**Rationale**: The codebase is small (~300 lines) and the breaking changes are well-documented. An incremental version-by-version upgrade would waste effort on intermediate APIs that are also deprecated. A direct jump is faster and cleaner.

**Key API changes to handle:**
- `NativeOptions { initial_window_size }` -> `NativeOptions { viewport: ViewportBuilder::default().with_inner_size([w, h]) }`
- `run_native` closure: `Box::new(|cc| Box::new(...))` -> `Box::new(|cc| Ok(Box::new(...)))`
- `Frame::canvas(style)` -> verify replacement in 0.33 (likely `Frame::canvas(style)` still exists or replaced by `Frame::new()` pattern)
- `Sense::drag()` -> verify still available (likely unchanged)

### 2. Pre-commit hooks: shell script in .githooks/
**Rationale**: Keeps the project pure Rust with no Node dependency. The streamer repo uses husky, but that requires Node/Yarn which is unnecessary for a pure Rust project.

**Implementation**: `.githooks/pre-commit` running `cargo fmt --check && cargo clippy -- -D warnings`. Developers activate with `git config core.hooksPath .githooks`.

**Alternative considered**: cargo-husky crate — rejected because it requires adding a dev-dependency and build script, more complexity than a simple shell script.

### 3. Releases: cargo-dist
**Rationale**: cargo-dist automates cross-platform binary builds and GitHub Releases with minimal configuration. It generates its own CI workflow, handles platform-specific packaging (.msi, .dmg, .tar.gz), and integrates with release-please.

**Alternative considered**: Manual build matrix (as in streamer's build.yml) — rejected because cargo-dist handles the same thing with less maintenance and better packaging out of the box.

### 4. Serde feature flag: define it properly in Cargo.toml
**Rationale**: Canvas already has `#[cfg_attr(feature = "serde", ...)]` attributes. Rather than removing them, we should define the feature properly since persistence is a planned capability. Enable it by default.

### 5. Release-please: release-type "rust"
**Rationale**: Mirrors the streamer repo's pattern but adapted for Rust. release-please has native Rust support that automatically bumps `version` in Cargo.toml.

### 6. New crates added as dependencies but not wired into application logic
**Rationale**: This change is about modernisation and infrastructure. The crates are declared so they're available and CI validates they compile, but feature implementation is a separate change.

## Risks / Trade-offs

- **[Risk] egui 0.33 API changes break canvas drawing** -> Mitigation: The canvas uses basic `Painter`, `Shape::line`, `Stroke`, `Sense::drag` which are stable across versions. `Frame::canvas` is the main risk; test after migration.
- **[Risk] cargo-dist generated workflow conflicts with manual workflows** -> Mitigation: cargo-dist manages its own `release.yml`; our manual `checks.yml` and `commit-lint.yml` are separate concerns with no overlap.
- **[Risk] Pre-commit hooks slow down commits** -> Mitigation: `cargo fmt --check` is near-instant on a small codebase; `cargo clippy` is fast with cached builds. Skip with `--no-verify` in emergencies (but per project conventions, fix the issue instead).
- **[Risk] dark-light 2.0 API changes** -> Mitigation: dark-light is imported but not yet wired into egui theming. The dependency update is safe; integration is a future task.
- **[Risk] Unused dependency warnings for newly added crates** -> Mitigation: Add a brief integration point or `use` statement for each crate, or accept the warnings temporarily since the next change will integrate them.
