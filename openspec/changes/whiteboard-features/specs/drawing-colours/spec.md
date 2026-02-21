## ADDED Requirements

### Requirement: Palette buttons set active stroke colour
The system SHALL update the active stroke colour in AppState when a palette button is clicked. The footer palette buttons SHALL provide visual feedback showing which colour is currently selected.

#### Scenario: User clicks a palette colour
- **WHEN** user clicks a palette colour button in the footer
- **THEN** the active stroke colour in AppState updates to that colour and the button displays a selected indicator (e.g., border highlight)

### Requirement: Each stroke stores its own colour
The system SHALL store the colour with each DrawObject at creation time. Previously drawn objects SHALL retain their original colour when the active colour changes.

#### Scenario: Drawing with different colours
- **WHEN** user draws a stroke with red, changes to blue, then draws another stroke
- **THEN** the first stroke remains red and the second stroke is blue

### Requirement: Default colour on startup
The system SHALL start with black (palette index 0) as the default active colour.

#### Scenario: Application launch
- **WHEN** the application starts
- **THEN** the first palette colour (black) is selected and new strokes draw in black
