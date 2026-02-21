## ADDED Requirements

### Requirement: Detect OS theme on startup
The system SHALL detect the OS theme (dark or light) on startup using the dark-light crate and apply matching egui Visuals.

#### Scenario: Dark mode OS
- **WHEN** the application starts on a system with dark mode enabled
- **THEN** egui uses `Visuals::dark()` styling

#### Scenario: Light mode OS
- **WHEN** the application starts on a system with light mode enabled
- **THEN** egui uses `Visuals::light()` styling

### Requirement: Manual theme toggle
The system SHALL provide a theme toggle button in the header to switch between dark and light mode at runtime.

#### Scenario: Toggling theme
- **WHEN** user clicks the theme toggle button
- **THEN** the UI switches between dark and light visuals immediately

### Requirement: Theme preference persists in session
The system SHALL remember the user's theme choice for the duration of the session.

#### Scenario: Theme stays after toggle
- **WHEN** user toggles to light mode
- **THEN** the UI remains in light mode until toggled again or the app is restarted
