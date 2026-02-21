## ADDED Requirements

### Requirement: Capture full screen
The system SHALL capture the full screen when the screenshot action is triggered. The captured image SHALL be loaded onto the canvas as a DrawObject::Image.

#### Scenario: Full screen capture
- **WHEN** user triggers screenshot capture
- **THEN** the Snap window minimises, the screen is captured, the window restores, and the image appears on the canvas

### Requirement: Screenshot action in header
The system SHALL provide a screenshot button in the header toolbar to trigger capture.

#### Scenario: Clicking screenshot button
- **WHEN** user clicks the screenshot button in the header
- **THEN** the screenshot capture flow begins (minimise → capture → restore → load)

### Requirement: Multi-monitor support
The system SHALL capture from the primary monitor by default. If multiple monitors are detected, the system SHALL capture from the monitor where the Snap window is displayed.

#### Scenario: Multi-monitor capture
- **WHEN** user triggers screenshot on a multi-monitor setup
- **THEN** the primary monitor's screen is captured

### Requirement: Region selection after capture
The system SHALL allow the user to select a rectangular region of the captured image to crop. The cropped region becomes the canvas content.

#### Scenario: Selecting a region
- **WHEN** a full screen capture is taken and the selection overlay appears
- **THEN** user can drag to select a rectangle, and only that region is kept on the canvas
