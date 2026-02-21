## Why

Snap's dependencies are severely outdated (egui/eframe 0.21 vs latest 0.33.3 — 12 minor versions behind) and the project lacks CI/CD, pre-commit hooks, tests, and essential crates for its core screenshot/whiteboard functionality. Modernising now establishes a solid foundation before feature development accelerates.

## What Changes

- **BREAKING**: Upgrade egui 0.21 -> 0.33.3 and eframe 0.21.3 -> 0.33.3 (API changes to `NativeOptions`, `run_native` closure signature, `Frame::canvas` usage)
- **BREAKING**: Upgrade dark-light 1.0 -> 2.0 (major version bump, API changes)
- Pin Rust toolchain via `rust-toolchain.toml` (1.93.0)
- Add new crates: `xcap` (screen capture), `global-hotkey` (system-wide shortcuts), `image` (image encoding/decoding), `serde` + `serde_json` (serialisation/persistence)
- Delete dead code: `main_using_iced.rs` (abandoned Iced prototype)
- Fix clippy errors in `footer.rs` (`format!("")` -> `String::new()`, `vec![...]` -> array literal)
- Fix serde feature flag: either define `serde` feature in Cargo.toml or remove unused `cfg_attr` from canvas.rs
- Set up `.githooks/pre-commit` shell script running `cargo fmt --check` and `cargo clippy -- -D warnings`
- Set up GitHub Actions CI: `checks.yml`, `build.yml`, `commit-lint.yml`, `lint-workflows.yml`, `release-please.yml`
- Set up `cargo-dist` for automated cross-platform release builds (Windows, Linux x64/ARM, macOS ARM/Intel)
- Add `.gitignore` for Rust target directory and IDE files

## Capabilities

### New Capabilities
- `ci-pipeline`: GitHub Actions workflows for quality gates (fmt, clippy, check, test), cross-platform builds, commit linting, workflow linting, and release-please automated releases
- `release-pipeline`: cargo-dist configuration for automated cross-platform binary releases (Windows .msi, Linux .deb, macOS .dmg) triggered by git tags
- `pre-commit-hooks`: Shell-based git hooks in `.githooks/` mirroring CI checks (fmt + clippy) to shift quality left
- `dependency-modernisation`: Upgraded egui/eframe to 0.33.3, dark-light to 2.0, and new crates (xcap, global-hotkey, image, serde) integrated and compiling

### Modified Capabilities
<!-- No existing specs to modify -->

## Impact

- **Cargo.toml**: All dependency versions change, new dependencies added, serde feature flag added
- **src/main.rs**: `NativeOptions` construction changes (`initial_window_size` -> `viewport`), `run_native` closure now returns `Result`
- **src/canvas.rs**: `Frame::canvas` usage may need updating, serde cfg_attr either enabled or removed
- **src/footer.rs**: Clippy fixes (2 lines)
- **src/main_using_iced.rs**: Deleted entirely
- **New files**: `rust-toolchain.toml`, `.githooks/pre-commit`, `.github/workflows/` (5 workflow files), `Cargo.toml` cargo-dist config, `.gitignore`
- **Build system**: cargo-dist adds release profile configuration and CI integration
- **Developer workflow**: Pre-commit hooks enforce fmt + clippy before every commit
