## Why

Snap currently has a canvas that draws freehand lines with a single hardcoded colour and stroke width. The palette buttons and eraser print to console but don't affect the canvas. None of the whiteboard or screenshot features work yet. To become a usable replacement for Snipping Tool and Microsoft Whiteboard, Snap needs functional drawing tools, editing capabilities, and screenshot/export workflows.

## What Changes

- Wire palette colour buttons to actually change the canvas stroke colour
- Add a working eraser that removes strokes
- Add adjustable line thickness (stroke width control)
- Add shape selection — click to select, move, and delete drawn shapes
- Add undo/redo with Ctrl+Z / Ctrl+Y history stack
- Add shape drawing tools: rectangle, ellipse, straight line, arrow
- Add screenshot capture using the xcap crate to capture screen regions
- Wire up dark-light crate to detect and apply OS theme (dark/light mode)
- Add export to PNG — save the canvas as an image file
- Add text annotations — place and edit text on the canvas
- Add clipboard copy/paste — copy canvas content to clipboard, paste images onto canvas

## Capabilities

### New Capabilities

- `drawing-colours`: Palette buttons change the active stroke colour; each stroke stores its own colour
- `eraser`: Eraser tool that removes strokes the pointer passes over
- `line-thickness`: UI control for stroke width; each stroke stores its own thickness
- `shape-selection`: Select, move, and delete drawn shapes on the canvas
- `undo-redo`: Ctrl+Z / Ctrl+Y history stack for all canvas operations
- `shape-tools`: Rectangle, ellipse, straight line, and arrow drawing tools
- `screenshot-capture`: Capture screen regions using xcap and load them onto the canvas
- `theme-switching`: Detect OS theme via dark-light crate and apply matching egui visuals
- `export-png`: Save the current canvas content as a PNG image file
- `text-annotations`: Place, edit, and style text labels on the canvas
- `clipboard`: Copy canvas/selection to system clipboard; paste images onto canvas

### Modified Capabilities

_(none — no existing specs)_

## Impact

- **canvas.rs**: Major refactor — strokes need per-stroke colour, width, and type; new data model for shapes, text, and images; selection and hit-testing logic; undo/redo command stack
- **footer.rs**: Wire colour buttons and eraser to shared app state; add thickness slider; add shape tool buttons
- **header.rs**: Add undo/redo buttons, export/screenshot actions, theme toggle
- **main.rs**: Introduce shared `AppState` struct passed to all components; keyboard shortcut handling
- **palette.rs**: Minor — already functional, just needs to communicate selected colour to canvas
- **New modules**: `tools.rs` (tool enum + behaviour), `history.rs` (undo/redo), `screenshot.rs` (xcap integration), `export.rs` (PNG save), `clipboard.rs` (system clipboard)
- **Dependencies**: All needed crates already in Cargo.toml (xcap, global-hotkey, image, dark-light). May need `arboard` crate for clipboard access.
