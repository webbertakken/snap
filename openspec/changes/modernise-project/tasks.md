## 1. Project housekeeping

- [ ] 1.1 Delete `src/main_using_iced.rs` (dead code)
- [ ] 1.2 Add `.gitignore` (target/, .idea/, .vscode/, *.swp, *.swo, .DS_Store, Thumbs.db, .openspec/)
- [ ] 1.3 Add `rust-toolchain.toml` pinning stable 1.93.0 with clippy and rustfmt components

## 2. Fix existing code issues

- [ ] 2.1 Fix clippy error in `footer.rs:39`: replace `format!("")` with `String::new()`
- [ ] 2.2 Fix clippy error in `footer.rs:30`: replace `vec![...]` with array literal `[...]`

## 3. Upgrade dependencies

- [ ] 3.1 Upgrade `egui` from 0.21.0 to 0.33.3 and `eframe` from 0.21.3 to 0.33.3 in Cargo.toml
- [ ] 3.2 Migrate `main.rs`: replace `NativeOptions { initial_window_size }` with `viewport: egui::ViewportBuilder::default().with_inner_size([1680.0, 1050.0])`
- [ ] 3.3 Migrate `main.rs`: update `run_native` closure to return `Result` — `Box::new(|cc| Ok(Box::new(...)))`
- [ ] 3.4 Migrate `canvas.rs`: verify and fix `Frame::canvas(ui.style())` usage for egui 0.33 API
- [ ] 3.5 Migrate any other breaking API changes found during compilation (e.g. `Sense`, `stroke_ui`, etc.)
- [ ] 3.6 Upgrade `dark-light` from 1.0.0 to 2.0.0 in Cargo.toml (verify compilation, no integration changes needed)

## 4. Add new dependencies

- [ ] 4.1 Add `xcap` (latest) to Cargo.toml
- [ ] 4.2 Add `global-hotkey` (latest) to Cargo.toml
- [ ] 4.3 Add `image` (latest) to Cargo.toml
- [ ] 4.4 Add `serde` with `derive` feature and `serde_json` to Cargo.toml
- [ ] 4.5 Define `serde` feature flag in `[features]` section; make serde an optional dependency; fix `cfg_attr` in `canvas.rs` to compile cleanly with and without the feature

## 5. Verify everything compiles and passes

- [ ] 5.1 Run `cargo fmt --check` — must pass
- [ ] 5.2 Run `cargo clippy -- -D warnings` — must pass with zero warnings
- [ ] 5.3 Run `cargo check` — must pass
- [ ] 5.4 Run `cargo test` — must pass
- [ ] 5.5 Run `cargo run` — verify app launches, window renders, drawing works

## 6. Pre-commit hooks

- [ ] 6.1 Create `.githooks/pre-commit` shell script: `cargo fmt --check && cargo clippy -- -D warnings`
- [ ] 6.2 Make `.githooks/pre-commit` executable (`chmod +x`)
- [ ] 6.3 Document hook activation in CLAUDE.md or README: `git config core.hooksPath .githooks`

## 7. GitHub Actions CI

- [ ] 7.1 Create `.github/workflows/checks.yml`: fmt, clippy, check, test on PR + push to main (using dtolnay/rust-toolchain@stable, swatinem/rust-cache@v2)
- [ ] 7.2 Create `.github/workflows/build.yml`: cross-platform build matrix (Windows, Ubuntu x64, Ubuntu ARM, macOS ARM, macOS Intel) on PR + tags
- [ ] 7.3 Create `.github/workflows/commit-lint.yml`: enforce conventional commits on PR titles (amannn/action-semantic-pull-request@v5)
- [ ] 7.4 Create `.github/workflows/lint-workflows.yml`: actionlint on workflow file changes (reviewdog/action-actionlint@v1)
- [ ] 7.5 Create `.github/workflows/release-please.yml`: automated release PRs (googleapis/release-please-action@v4 with release-type: rust)
- [ ] 7.6 Create `release-please-config.json` and `.release-please-manifest.json`

## 8. cargo-dist release pipeline

- [ ] 8.1 Install cargo-dist locally and run `cargo dist init` to generate initial config
- [ ] 8.2 Configure cargo-dist for 5 target platforms (Windows x64, Linux x64, Linux ARM, macOS ARM, macOS Intel)
- [ ] 8.3 Verify cargo-dist generated workflow integrates with release-please tag flow
- [ ] 8.4 Test `cargo dist build` locally to verify it produces artefacts

## 9. Final verification

- [ ] 9.1 Run full quality check suite: `cargo fmt --check && cargo clippy -- -D warnings && cargo check && cargo test`
- [ ] 9.2 Verify pre-commit hook works: activate hooks, make a commit with intentional fmt issue, confirm rejection
- [ ] 9.3 Review all new/modified files for completeness
