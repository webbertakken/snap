## Context

Snap is a ~300-line Rust/egui desktop app with a basic freehand drawing canvas. The canvas stores lines as `Vec<Vec<Pos2>>` with a single shared `Stroke`. The footer has palette buttons and an eraser button that only `println!` — no state flows between components. There is no shared app state; each component owns its own data. egui is immediate-mode (full redraw every frame), so state must live outside the UI code and be passed in.

## Goals / Non-Goals

**Goals:**
- Introduce a shared `AppState` that all components read/write, enabling tool selection, colour, and thickness to flow from footer to canvas
- Store per-stroke metadata (colour, width, tool type) so strokes render independently
- Introduce a `DrawObject` enum (freehand, rect, ellipse, line, arrow, text, image) as the unified canvas element
- Add an undo/redo command stack operating on `DrawObject` additions/removals/mutations
- Add shape selection with hit-testing, bounding box handles, and keyboard delete
- Integrate xcap for screen capture, image crate for PNG export, and system clipboard via arboard
- Wire dark-light crate to set egui `Visuals` on startup and optionally toggle at runtime

**Non-Goals:**
- Layers or grouping — single flat list of draw objects for now
- Vector export (SVG) — only raster PNG export
- Multi-page or infinite canvas — single viewport with no zoom/pan
- Collaborative/multi-user features
- Plugin or extension system
- Touch gesture handling beyond basic pointer events

## Decisions

### 1. Shared AppState struct
**Decision:** Introduce `AppState` in `state.rs` holding active tool, active colour, stroke width, theme, and a `Vec<DrawObject>` for canvas contents. Pass `&mut AppState` to all component `render()` calls.
**Rationale:** egui is immediate-mode — there's no reactive binding. A single mutable state struct is the idiomatic pattern. Alternatives (message passing, channels) add complexity without benefit for a single-threaded GUI.

### 2. DrawObject enum as unified canvas element
**Decision:** Replace `Vec<Vec<Pos2>>` with `Vec<DrawObject>` where `DrawObject` is an enum: `Freehand { points, colour, width }`, `Rectangle { rect, colour, width, filled }`, `Ellipse { ... }`, `Line { start, end, colour, width }`, `Arrow { ... }`, `Text { pos, content, font_size, colour }`, `Image { pos, texture_handle, size }`.
**Rationale:** A unified enum lets selection, undo/redo, serialisation, and rendering operate generically over all shape types. Using trait objects would complicate serialisation and cloning.

### 3. Command-based undo/redo
**Decision:** Use a `Vec<Command>` history stack with a cursor index. Commands: `Add(DrawObject)`, `Remove(index)`, `Modify(index, old, new)`. Undo replays in reverse; redo replays forward.
**Rationale:** Command pattern is standard for undo/redo. Storing full snapshots would waste memory for large drawings. The command list also enables future "replay" features.

### 4. Hit-testing for selection
**Decision:** Implement per-DrawObject `contains(pos) -> bool` and `bounding_rect() -> Rect`. For freehand strokes, use distance-to-polyline with a tolerance threshold. For shapes, use geometric containment. Selected objects get a dashed bounding box overlay with drag handles.
**Rationale:** Per-object hit-testing is simple and fast for the expected object counts (<1000). Spatial indexing (R-tree) would be premature.

### 5. Eraser via stroke removal
**Decision:** Eraser tool checks pointer position against all objects using hit-testing. Objects the pointer passes over are removed (added to undo stack as `Remove` commands). This is whole-object erasure, not pixel-level.
**Rationale:** Whole-object erasure is simpler and matches the vector-based data model. Pixel-level erasure would require rasterisation.

### 6. Screenshot capture flow
**Decision:** Minimise Snap window → capture with xcap → restore window → load captured image as `DrawObject::Image` on canvas. Use `xcap::Monitor::all()` for multi-monitor support and `Monitor::capture_image()` for the actual grab.
**Rationale:** xcap is already a dependency. The minimise/capture/restore flow is standard for screenshot tools. Region selection will overlay a translucent selection rect on the captured image.

### 7. Theme detection and switching
**Decision:** Call `dark_light::detect()` at startup to set initial `egui::Visuals`. Add a toggle button in the header. Store theme preference in AppState.
**Rationale:** dark-light is already imported. egui has built-in `Visuals::dark()` and `Visuals::light()`.

### 8. Clipboard via arboard crate
**Decision:** Add `arboard` dependency for cross-platform clipboard. Copy renders the selection (or full canvas) to an image buffer and copies to clipboard. Paste reads clipboard image and creates `DrawObject::Image`.
**Rationale:** egui's built-in clipboard is text-only. `arboard` handles image clipboard on Windows, macOS, and Linux (X11/Wayland).

### 9. PNG export via image crate
**Decision:** Render all DrawObjects to an off-screen buffer using egui's `ColorImage`, then save via `image::save_buffer()`. Use `rfd` (or native file dialog via egui) for save path selection.
**Rationale:** The `image` crate is already a dependency. Off-screen rendering avoids depending on the current viewport state.

### 10. Module structure
**Decision:** New modules: `state.rs` (AppState + DrawObject + Tool enum), `history.rs` (undo/redo stack), `tools/` directory with `mod.rs`, `freehand.rs`, `shapes.rs`, `text.rs`, `eraser.rs`, `selection.rs`. Keep `screenshot.rs`, `export.rs`, `clipboard.rs` as separate modules.
**Rationale:** Domain-driven structure per AGENTS.md. Each tool encapsulates its own input handling and rendering logic.

## Risks / Trade-offs

- **Off-screen rendering for export** — egui doesn't natively support off-screen rendering. May need to render to a texture and read back pixels, which is GPU-dependent. → Mitigation: For MVP, screenshot the canvas panel area; full off-screen rendering can be a follow-up.
- **Clipboard image format** — arboard's image support varies by platform. → Mitigation: Test on Windows first (primary target), degrade gracefully on other platforms.
- **Screenshot on Wayland** — xcap may have limited Wayland support. → Mitigation: Primary target is Windows; Linux support via X11 initially.
- **Performance with many objects** — Rendering hundreds of shapes every frame in immediate-mode. → Mitigation: egui handles this well for <1000 shapes. Optimise later if needed.
- **Breaking change to canvas data model** — Replacing `Vec<Vec<Pos2>>` with `Vec<DrawObject>` is a complete rewrite of canvas.rs. → Mitigation: All features depend on this change; it's the necessary foundation.
