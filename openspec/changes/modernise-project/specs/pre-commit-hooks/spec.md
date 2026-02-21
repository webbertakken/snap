## ADDED Requirements

### Requirement: Pre-commit hook runs format and lint checks
A git pre-commit hook at `.githooks/pre-commit` SHALL run `cargo fmt --check` and `cargo clippy -- -D warnings` before every commit. The hook MUST mirror the CI quality checks.

#### Scenario: Code with formatting issues
- **WHEN** a developer attempts to commit code that fails `cargo fmt --check`
- **THEN** the pre-commit hook SHALL reject the commit with an error message

#### Scenario: Code with clippy warnings
- **WHEN** a developer attempts to commit code that triggers clippy warnings
- **THEN** the pre-commit hook SHALL reject the commit with an error message

#### Scenario: Clean code
- **WHEN** a developer attempts to commit code that passes both fmt and clippy
- **THEN** the pre-commit hook SHALL allow the commit to proceed

### Requirement: Hook activation via git config
The project SHALL document and support activating pre-commit hooks via `git config core.hooksPath .githooks`. This MUST NOT require Node.js, Python, or any non-Rust tooling.

#### Scenario: Developer activates hooks
- **WHEN** a developer runs `git config core.hooksPath .githooks` in the repository
- **THEN** all subsequent commits SHALL trigger the pre-commit hook

#### Scenario: No automatic activation
- **WHEN** a developer clones the repository without running the git config command
- **THEN** no pre-commit hooks SHALL run (opt-in, not mandatory)
