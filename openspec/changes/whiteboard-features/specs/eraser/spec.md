## ADDED Requirements

### Requirement: Eraser tool removes whole objects
The system SHALL provide an eraser tool that removes entire DrawObjects when the pointer passes over them. The eraser SHALL use hit-testing with a configurable tolerance radius.

#### Scenario: Erasing a freehand stroke
- **WHEN** the eraser tool is active and the user drags the pointer over a freehand stroke
- **THEN** the stroke is removed from the canvas and a Remove command is pushed to the undo stack

#### Scenario: Erasing a shape
- **WHEN** the eraser tool is active and the user clicks on a rectangle or ellipse
- **THEN** the shape is removed from the canvas

### Requirement: Eraser button in footer activates eraser tool
The system SHALL include an eraser button in the footer toolbar. Clicking it SHALL set the active tool to Eraser.

#### Scenario: Switching to eraser
- **WHEN** user clicks the eraser button (E) in the footer
- **THEN** the active tool changes to Eraser and the eraser button shows a selected indicator

### Requirement: Eraser cursor visual feedback
The system SHALL display a circle cursor indicating the eraser radius when the eraser tool is active.

#### Scenario: Eraser active on canvas
- **WHEN** the eraser tool is active and the pointer is over the canvas
- **THEN** a circle is drawn at the pointer position showing the erase radius
