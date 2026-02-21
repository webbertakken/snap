## ADDED Requirements

### Requirement: Selection tool selects objects on click
The system SHALL provide a selection tool. When active, clicking on a DrawObject SHALL select it, displaying a bounding box with handles.

#### Scenario: Clicking on a shape
- **WHEN** selection tool is active and user clicks on a drawn rectangle
- **THEN** the rectangle is selected with a visible bounding box around it

#### Scenario: Clicking on empty canvas
- **WHEN** selection tool is active and user clicks on an empty area
- **THEN** any current selection is cleared

### Requirement: Move selected objects
The system SHALL allow dragging a selected object to move it to a new position. The move SHALL be recorded as a Modify command in the undo stack.

#### Scenario: Dragging a selected shape
- **WHEN** user drags a selected object
- **THEN** the object moves with the pointer and its position updates in AppState

### Requirement: Delete selected objects
The system SHALL delete selected objects when the Delete or Backspace key is pressed.

#### Scenario: Deleting a selection
- **WHEN** an object is selected and the user presses Delete
- **THEN** the object is removed from the canvas and a Remove command is pushed to the undo stack

### Requirement: Selection visual feedback
The system SHALL render a dashed bounding rectangle around selected objects with distinct styling (e.g., blue dashed border).

#### Scenario: Object selected
- **WHEN** an object is selected
- **THEN** a dashed blue rectangle is drawn around the object's bounding box
