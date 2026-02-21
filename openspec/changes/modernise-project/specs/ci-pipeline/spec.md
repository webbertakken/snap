## ADDED Requirements

### Requirement: Quality gate checks on every PR and push to main
The CI system SHALL run `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo check`, and `cargo test` on every pull request and every push to the `main` branch. All checks MUST pass before a PR can be merged.

#### Scenario: PR with formatting violation
- **WHEN** a pull request contains code that fails `cargo fmt --check`
- **THEN** the checks workflow SHALL fail and block merging

#### Scenario: PR with clippy warning
- **WHEN** a pull request contains code that triggers a clippy warning
- **THEN** the checks workflow SHALL fail (clippy runs with `-D warnings`)

#### Scenario: All checks pass
- **WHEN** a pull request passes fmt, clippy, check, and test
- **THEN** the checks workflow SHALL succeed and allow merging

### Requirement: Cross-platform build verification on every PR
The CI system SHALL build the project on all target platforms (Windows, Ubuntu x64, Ubuntu ARM, macOS ARM, macOS Intel) for every pull request to verify cross-platform compilation.

#### Scenario: PR build matrix
- **WHEN** a pull request is opened or updated
- **THEN** the build workflow SHALL compile the project on all 5 platform targets with `fail-fast: false`

#### Scenario: Platform-specific build failure
- **WHEN** the build fails on one platform but succeeds on others
- **THEN** all platform builds SHALL still run to completion (fail-fast disabled) and the failing platform SHALL be reported

### Requirement: Conventional commit enforcement on PRs
The CI system SHALL enforce conventional commit format on pull request titles using `amannn/action-semantic-pull-request`. Accepted types: feat, fix, docs, style, refactor, perf, test, build, ci, chore, revert.

#### Scenario: Non-conventional PR title
- **WHEN** a pull request has a title like "updated stuff"
- **THEN** the commit-lint workflow SHALL fail

#### Scenario: Conventional PR title
- **WHEN** a pull request has a title like "feat: add screenshot capture"
- **THEN** the commit-lint workflow SHALL pass

### Requirement: Workflow file linting
The CI system SHALL lint GitHub Actions workflow files using `reviewdog/action-actionlint` whenever workflow files are modified in a PR.

#### Scenario: Invalid workflow syntax
- **WHEN** a PR modifies a file in `.github/workflows/` with invalid YAML or action syntax
- **THEN** the lint-workflows job SHALL fail and report the issue

#### Scenario: No workflow changes
- **WHEN** a PR does not modify any files in `.github/workflows/`
- **THEN** the lint-workflows job SHALL not run (path filter)

### Requirement: Rust build caching
The CI system SHALL use `swatinem/rust-cache@v2` to cache Rust build artefacts across CI runs to reduce build times.

#### Scenario: Cached build
- **WHEN** a CI run starts with a warm cache from a previous run
- **THEN** the build SHALL reuse cached compilation artefacts and complete faster than a cold build

### Requirement: Rust toolchain setup
The CI system SHALL use `dtolnay/rust-toolchain@stable` with `clippy` and `rustfmt` components to ensure consistent toolchain across all CI runs.

#### Scenario: Toolchain installation
- **WHEN** a CI workflow starts
- **THEN** the stable Rust toolchain SHALL be installed with clippy and rustfmt components available
