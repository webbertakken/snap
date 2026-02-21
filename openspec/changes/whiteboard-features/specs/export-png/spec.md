## ADDED Requirements

### Requirement: Export canvas as PNG
The system SHALL export the current canvas content as a PNG image file. A file save dialog SHALL let the user choose the destination path.

#### Scenario: Exporting to PNG
- **WHEN** user triggers export and selects a save path
- **THEN** the canvas contents are rendered to a PNG file at the chosen path

### Requirement: Export action in header
The system SHALL provide an export/save button in the header toolbar.

#### Scenario: Clicking export button
- **WHEN** user clicks the export button in the header
- **THEN** a file save dialog opens with PNG as the default format

### Requirement: Export includes all visible objects
The system SHALL include all DrawObjects currently on the canvas in the exported image, rendered at their current positions and styles.

#### Scenario: Exporting with multiple shapes
- **WHEN** canvas has freehand strokes, rectangles, and text, and user exports
- **THEN** the PNG file contains all objects rendered correctly

### Requirement: Export respects canvas background
The system SHALL use the current canvas background colour (matching the active theme) as the PNG background.

#### Scenario: Export in dark mode
- **WHEN** user exports while in dark mode
- **THEN** the PNG has a dark background matching the canvas
