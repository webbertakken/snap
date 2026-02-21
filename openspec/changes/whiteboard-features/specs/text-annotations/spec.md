## ADDED Requirements

### Requirement: Text tool places editable text on canvas
The system SHALL provide a text tool. When active, clicking on the canvas SHALL create an inline text editing area at that position. The user types text and it becomes a DrawObject::Text on the canvas.

#### Scenario: Adding text
- **WHEN** text tool is active and user clicks on the canvas
- **THEN** an inline text input appears at the click position

#### Scenario: Confirming text
- **WHEN** user presses Enter or clicks away from the text input
- **THEN** the typed text becomes a permanent DrawObject::Text on the canvas using the active colour

### Requirement: Text uses active colour and configurable size
The system SHALL render text in the active stroke colour. Font size SHALL use the current stroke width as a scaling factor (e.g., width * 6 for font size).

#### Scenario: Text colour matches active colour
- **WHEN** user selects red from palette and adds text
- **THEN** the text renders in red

### Requirement: Text tool button in footer
The system SHALL include a text tool button in the footer toolbar.

#### Scenario: Switching to text tool
- **WHEN** user clicks the text tool button
- **THEN** the active tool changes to Text

### Requirement: Select and move text
Text objects SHALL be selectable and movable via the selection tool, like any other DrawObject.

#### Scenario: Moving text with selection tool
- **WHEN** selection tool is active and user drags a text object
- **THEN** the text moves to the new position
