## ADDED Requirements

### Requirement: Rust toolchain pinned via rust-toolchain.toml
The project SHALL include a `rust-toolchain.toml` file pinning the Rust toolchain to stable channel version 1.93.0 with clippy and rustfmt components.

#### Scenario: Developer with different default toolchain
- **WHEN** a developer with a different default Rust version enters the project directory
- **THEN** rustup SHALL automatically use the pinned toolchain version

### Requirement: egui and eframe upgraded to 0.33.3
The project SHALL use egui 0.33.3 and eframe 0.33.3. All API changes MUST be migrated:
- `NativeOptions::initial_window_size` replaced with `viewport: egui::ViewportBuilder::default().with_inner_size([1680.0, 1050.0])`
- `run_native` closure returns `Result`: `Box::new(|cc| Ok(Box::new(...)))`
- Any deprecated APIs (e.g. `Frame::canvas`) replaced with current equivalents

#### Scenario: Application starts successfully
- **WHEN** the application is launched with `cargo run`
- **THEN** the window SHALL open at 1680x1050 with the same layout as before (header, footer, side panel, canvas)

#### Scenario: Drawing still works
- **WHEN** a user draws on the canvas after the upgrade
- **THEN** freehand lines SHALL render correctly using the same normalised coordinate system

### Requirement: dark-light upgraded to 2.0
The project SHALL use dark-light 2.0. The crate is imported but not yet wired into the UI; the upgrade MUST NOT break compilation.

#### Scenario: Compilation succeeds
- **WHEN** the project is compiled with dark-light 2.0
- **THEN** `cargo check` SHALL pass without errors

### Requirement: New crates added to Cargo.toml
The project SHALL add the following dependencies: `xcap` (latest), `global-hotkey` (latest), `image` (latest), `serde` with `derive` feature, and `serde_json` (latest). These crates MUST compile but are not required to be integrated into application logic.

#### Scenario: All new crates compile
- **WHEN** the project is compiled with all new dependencies
- **THEN** `cargo check` SHALL pass without errors

### Requirement: Serde feature flag properly defined
The `serde` feature SHALL be defined in `Cargo.toml` under `[features]` and SHALL enable `serde` as an optional dependency. The `cfg_attr` attributes on the `Canvas` struct SHALL work correctly when the feature is enabled.

#### Scenario: Serde feature enabled
- **WHEN** the project is compiled with `cargo check --features serde`
- **THEN** the `Canvas` struct SHALL derive `serde::Serialize` and `serde::Deserialize`

#### Scenario: Default compilation without serde feature
- **WHEN** the project is compiled with `cargo check` (no features)
- **THEN** no serde-related warnings SHALL appear

### Requirement: Dead code removed
The file `src/main_using_iced.rs` SHALL be deleted. It is an abandoned Iced prototype that is not compiled or referenced.

#### Scenario: File deleted
- **WHEN** the source tree is inspected after this change
- **THEN** `src/main_using_iced.rs` SHALL not exist

### Requirement: Clippy errors in footer.rs fixed
The two clippy errors in `footer.rs` SHALL be fixed:
- `format!("")` on line 39 replaced with `String::new()`
- `vec!["...", "...", "..."]` on line 30 replaced with an array literal `["...", "...", "..."]`

#### Scenario: Clippy passes
- **WHEN** `cargo clippy -- -D warnings` is run
- **THEN** zero errors and zero warnings SHALL be reported

### Requirement: .gitignore file added
The project SHALL include a `.gitignore` file ignoring at minimum: `target/`, IDE files (`.idea/`, `.vscode/`, `*.swp`, `*.swo`), and OS files (`.DS_Store`, `Thumbs.db`).

#### Scenario: Build artefacts not tracked
- **WHEN** `cargo build` creates the `target/` directory
- **THEN** git SHALL not show `target/` files as untracked

### Requirement: All quality checks pass
After all changes, `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo check`, and `cargo test` SHALL all pass with zero errors and zero warnings.

#### Scenario: Full quality check suite
- **WHEN** all modernisation changes are applied
- **THEN** running `cargo fmt --check && cargo clippy -- -D warnings && cargo check && cargo test` SHALL exit with code 0
