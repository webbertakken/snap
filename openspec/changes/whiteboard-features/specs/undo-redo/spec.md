## ADDED Requirements

### Requirement: Undo last action with Ctrl+Z
The system SHALL undo the last canvas operation when Ctrl+Z is pressed. Undone operations SHALL be available for redo.

#### Scenario: Undoing a drawn stroke
- **WHEN** user draws a stroke and presses Ctrl+Z
- **THEN** the stroke is removed from the canvas and the undo stack cursor moves back

#### Scenario: Nothing to undo
- **WHEN** user presses Ctrl+Z with an empty history
- **THEN** nothing happens (no crash, no visual change)

### Requirement: Redo with Ctrl+Y
The system SHALL redo the last undone operation when Ctrl+Y is pressed.

#### Scenario: Redoing an undone stroke
- **WHEN** user undoes a stroke with Ctrl+Z then presses Ctrl+Y
- **THEN** the stroke reappears on the canvas

### Requirement: New action clears redo stack
The system SHALL clear all redo entries when a new action is performed after an undo.

#### Scenario: Drawing after undo
- **WHEN** user undoes an action then draws a new stroke
- **THEN** the undone action is no longer available for redo

### Requirement: Undo/redo buttons in header
The system SHALL display undo and redo buttons in the header toolbar as a secondary access method.

#### Scenario: Clicking undo button
- **WHEN** user clicks the undo button in the header
- **THEN** the last action is undone, same as Ctrl+Z
