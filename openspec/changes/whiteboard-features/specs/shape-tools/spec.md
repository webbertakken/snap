## ADDED Requirements

### Requirement: Rectangle drawing tool
The system SHALL provide a rectangle tool. When active, click-and-drag SHALL create a rectangle from the start point to the release point, using the active colour and stroke width.

#### Scenario: Drawing a rectangle
- **WHEN** rectangle tool is active and user click-drags from point A to point B
- **THEN** a rectangle is drawn with corners at A and B, using the active colour and width

#### Scenario: Rectangle preview while dragging
- **WHEN** user is mid-drag with the rectangle tool
- **THEN** a preview rectangle is shown following the pointer

### Requirement: Ellipse drawing tool
The system SHALL provide an ellipse tool. Click-and-drag SHALL create an ellipse inscribed in the rectangle from start to release point.

#### Scenario: Drawing an ellipse
- **WHEN** ellipse tool is active and user click-drags from point A to point B
- **THEN** an ellipse inscribed in the bounding rect A→B is drawn with active colour and width

### Requirement: Straight line tool
The system SHALL provide a line tool. Click-and-drag SHALL draw a straight line from start to release point.

#### Scenario: Drawing a straight line
- **WHEN** line tool is active and user click-drags from point A to point B
- **THEN** a straight line from A to B is drawn with active colour and width

### Requirement: Arrow tool
The system SHALL provide an arrow tool. Click-and-drag SHALL draw a line with an arrowhead at the end point.

#### Scenario: Drawing an arrow
- **WHEN** arrow tool is active and user click-drags from point A to point B
- **THEN** a line with an arrowhead pointing at B is drawn with active colour and width

### Requirement: Shape tool buttons in footer
The system SHALL display tool buttons for rectangle, ellipse, line, and arrow in the footer toolbar alongside the existing freehand tools.

#### Scenario: Switching to rectangle tool
- **WHEN** user clicks the rectangle tool button
- **THEN** the active tool changes to Rectangle and the button shows a selected indicator
