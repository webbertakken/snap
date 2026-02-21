## 1. Foundation — shared state and draw object model

- [ ] 1.1 Create `state.rs` with `AppState` struct (active tool, active colour, stroke width, theme, objects list)
- [ ] 1.2 Create `DrawObject` enum with variants: Freehand, Rectangle, Ellipse, Line, Arrow, Text, Image — each with per-object colour and width
- [ ] 1.3 Create `Tool` enum: Freehand, Rectangle, Ellipse, Line, Arrow, Text, Eraser, Selection
- [ ] 1.4 Refactor `main.rs` to create AppState and pass `&mut AppState` to all component render methods
- [ ] 1.5 Refactor `canvas.rs` to use `Vec<DrawObject>` from AppState instead of local `Vec<Vec<Pos2>>`
- [ ] 1.6 Refactor `footer.rs` to read/write AppState (active tool, colour, width) instead of `println!`

## 2. Drawing colours

- [ ] 2.1 Wire palette button clicks to set `app_state.active_colour`
- [ ] 2.2 Add selected indicator (border highlight) on the active palette button
- [ ] 2.3 Store active colour in each new DrawObject at creation time
- [ ] 2.4 Render each DrawObject with its stored colour (not the global stroke)

## 3. Line thickness

- [ ] 3.1 Add thickness preset buttons (1px, 2px, 4px, 8px) to footer toolbar
- [ ] 3.2 Wire thickness buttons to set `app_state.stroke_width`
- [ ] 3.3 Add selected indicator on active thickness preset
- [ ] 3.4 Store active width in each new DrawObject at creation time
- [ ] 3.5 Render each DrawObject with its stored width

## 4. Eraser

- [ ] 4.1 Create `eraser.rs` module with hit-test-based erasure logic
- [ ] 4.2 Implement distance-to-polyline hit-testing for Freehand strokes
- [ ] 4.3 Implement bounding-rect hit-testing for shape DrawObjects
- [ ] 4.4 Wire eraser button in footer to set active tool to Eraser
- [ ] 4.5 Remove objects the pointer passes over when eraser is active
- [ ] 4.6 Draw circle cursor showing erase radius

## 5. Undo / Redo

- [ ] 5.1 Create `history.rs` with Command enum (Add, Remove, Modify) and history stack with cursor
- [ ] 5.2 Push Add commands when new DrawObjects are created
- [ ] 5.3 Push Remove commands when objects are erased or deleted
- [ ] 5.4 Implement undo (Ctrl+Z) — reverse last command
- [ ] 5.5 Implement redo (Ctrl+Y) — replay next command
- [ ] 5.6 Clear redo stack on new action after undo
- [ ] 5.7 Add undo/redo buttons to header toolbar

## 6. Shape tools

- [ ] 6.1 Create `tools/shapes.rs` with shared click-drag-release input handling
- [ ] 6.2 Implement rectangle tool — preview during drag, create DrawObject::Rectangle on release
- [ ] 6.3 Implement ellipse tool — preview during drag, create DrawObject::Ellipse on release
- [ ] 6.4 Implement straight line tool — preview during drag, create DrawObject::Line on release
- [ ] 6.5 Implement arrow tool — line with arrowhead, preview during drag
- [ ] 6.6 Add shape tool buttons (rect, ellipse, line, arrow) to footer toolbar

## 7. Shape selection

- [ ] 7.1 Create `tools/selection.rs` with selection state (selected object index)
- [ ] 7.2 Implement click-to-select using hit-testing (reuse eraser hit-test logic)
- [ ] 7.3 Render dashed bounding box around selected object
- [ ] 7.4 Implement drag-to-move for selected objects (push Modify command to history)
- [ ] 7.5 Implement Delete/Backspace to remove selected object
- [ ] 7.6 Clear selection on click-on-empty or tool switch

## 8. Screenshot capture

- [ ] 8.1 Create `screenshot.rs` module wrapping xcap capture
- [ ] 8.2 Implement minimise → capture → restore → load flow
- [ ] 8.3 Convert xcap image to egui TextureHandle and create DrawObject::Image
- [ ] 8.4 Add screenshot button to header toolbar
- [ ] 8.5 Implement region selection overlay after capture (drag to crop)

## 9. Dark/light theme

- [ ] 9.1 Call `dark_light::detect()` on startup and set egui Visuals accordingly
- [ ] 9.2 Add theme toggle button to header toolbar
- [ ] 9.3 Store theme preference in AppState and apply on toggle

## 10. Export to PNG

- [ ] 10.1 Create `export.rs` module with canvas-to-image rendering
- [ ] 10.2 Render all DrawObjects to a pixel buffer (ColorImage)
- [ ] 10.3 Save pixel buffer as PNG using the image crate
- [ ] 10.4 Add export button to header with file save dialog (rfd or native)
- [ ] 10.5 Use current theme background colour for PNG background

## 11. Text annotations

- [ ] 11.1 Create `tools/text.rs` with text input state machine (placing → editing → committed)
- [ ] 11.2 Show inline text input at click position when text tool is active
- [ ] 11.3 Commit typed text as DrawObject::Text on Enter or click-away
- [ ] 11.4 Render text using active colour with font size derived from stroke width
- [ ] 11.5 Add text tool button to footer toolbar

## 12. Clipboard

- [ ] 12.1 Add `arboard` dependency to Cargo.toml
- [ ] 12.2 Create `clipboard.rs` module wrapping arboard for image clipboard
- [ ] 12.3 Implement Ctrl+C — render selection or full canvas to image, copy to clipboard
- [ ] 12.4 Implement Ctrl+V — read clipboard image, create DrawObject::Image on canvas
