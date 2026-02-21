## ADDED Requirements

### Requirement: Automated release PR creation via release-please
The release system SHALL use `googleapis/release-please-action@v4` with `release-type: "rust"` to automatically create release PRs from conventional commits pushed to `main`. The release PR SHALL update the version in `Cargo.toml` and maintain a `CHANGELOG.md`.

#### Scenario: Feature commit pushed to main
- **WHEN** a commit with prefix `feat:` is pushed to the `main` branch
- **THEN** release-please SHALL create or update a release PR with a minor version bump and updated CHANGELOG

#### Scenario: Fix commit pushed to main
- **WHEN** a commit with prefix `fix:` is pushed to the `main` branch
- **THEN** release-please SHALL create or update a release PR with a patch version bump and updated CHANGELOG

#### Scenario: Release PR merged
- **WHEN** a release-please PR is merged
- **THEN** release-please SHALL create a git tag `v<version>` which triggers the release build

### Requirement: Cross-platform binary releases via cargo-dist
The release system SHALL use cargo-dist to build and publish release binaries for Windows (x86_64), Linux (x86_64, aarch64), and macOS (aarch64, x86_64) when a version tag is pushed.

#### Scenario: Version tag pushed
- **WHEN** a git tag matching `v*` is pushed
- **THEN** cargo-dist SHALL build release binaries for all 5 platform targets and create a GitHub Release with downloadable assets

#### Scenario: Release assets
- **WHEN** a GitHub Release is created by cargo-dist
- **THEN** the release SHALL contain platform-appropriate installers/archives (e.g. .msi for Windows, .tar.gz for Linux, .dmg for macOS)

### Requirement: Release token for tag-triggered builds
The release system SHALL use a custom `RELEASE_TOKEN` secret (not the default `GITHUB_TOKEN`) for release-please so that the tag push triggers downstream build workflows.

#### Scenario: Tag triggers build
- **WHEN** release-please creates a tag using `RELEASE_TOKEN`
- **THEN** the cargo-dist release workflow SHALL be triggered by the tag event
