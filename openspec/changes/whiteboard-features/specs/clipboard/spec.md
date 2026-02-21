## ADDED Requirements

### Requirement: Copy selection to clipboard
The system SHALL copy the selected object(s) or full canvas as an image to the system clipboard when Ctrl+C is pressed.

#### Scenario: Copying a selected object
- **WHEN** an object is selected and user presses Ctrl+C
- **THEN** the selected object is rendered as an image and placed on the system clipboard

#### Scenario: Copying with no selection
- **WHEN** no object is selected and user presses Ctrl+C
- **THEN** the entire canvas is rendered as an image and placed on the system clipboard

### Requirement: Paste image from clipboard
The system SHALL paste clipboard image content onto the canvas as a DrawObject::Image when Ctrl+V is pressed.

#### Scenario: Pasting an image
- **WHEN** the system clipboard contains an image and user presses Ctrl+V
- **THEN** the image appears on the canvas as a new DrawObject::Image at the centre of the viewport

#### Scenario: Pasting with no image in clipboard
- **WHEN** the clipboard does not contain an image and user presses Ctrl+V
- **THEN** nothing happens (no crash, no visual change)

### Requirement: Clipboard uses arboard crate
The system SHALL use the `arboard` crate for cross-platform clipboard image support.

#### Scenario: Clipboard on Windows
- **WHEN** user copies or pastes on Windows
- **THEN** the system clipboard is accessed via arboard with image support
