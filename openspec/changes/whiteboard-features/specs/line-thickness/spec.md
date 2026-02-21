## ADDED Requirements

### Requirement: Stroke width control in footer
The system SHALL display a thickness slider or preset buttons in the footer toolbar. The control SHALL update the active stroke width in AppState.

#### Scenario: Adjusting line thickness
- **WHEN** user changes the thickness control to 4px
- **THEN** the active stroke width updates to 4.0 and new strokes draw at that width

### Requirement: Each stroke stores its own width
The system SHALL store the stroke width with each DrawObject at creation time. Changing thickness SHALL NOT affect previously drawn objects.

#### Scenario: Drawing with different thicknesses
- **WHEN** user draws a stroke at 2px, changes to 6px, then draws another
- **THEN** the first stroke remains 2px and the second is 6px

### Requirement: Default stroke width
The system SHALL start with a default stroke width of 2px.

#### Scenario: Application launch
- **WHEN** the application starts
- **THEN** the stroke width is set to 2.0px

### Requirement: Thickness presets
The system SHALL offer at least 4 thickness presets: thin (1px), medium (2px), thick (4px), extra-thick (8px).

#### Scenario: Selecting a thickness preset
- **WHEN** user clicks the "thick" preset button
- **THEN** the active stroke width changes to 4.0px
