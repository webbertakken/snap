# Architecture analysis for Snap feature development

## Current data model and state management

### Struct hierarchy
```
Snap (App)
  ├── header: Header          — stateless, placeholder labels
  ├── canvas: Canvas          — owns all drawing state
  │     ├── lines: Vec<Vec<Pos2>>   — strokes in normalised 0..1 coords
  │     └── stroke: Stroke          — current stroke style (width + colour)
  └── footer: Footer          — owns toolbar UI state
        ├── center_widget: Widget   — centering utility
        └── palette: Palette        — 10 hardcoded Color32 values (fields color0..color9)
```

### Canvas data model (`canvas.rs`)
- **Strokes** are stored as `Vec<Vec<Pos2>>` — each inner `Vec<Pos2>` is one continuous freehand line.
- Points are in **normalised 0..1 coordinates** using `RectTransform::from_to` between a unit square (with `square_proportions()`) and screen space.
- A new empty `Vec<Pos2>` is pushed when the pointer lifts (drag ends).
- **Current stroke style**: a single `egui::Stroke` (width: `f32`, colour: `Color32`), initialised to `Stroke::new(1.0, Color32::from_rgb(25, 200, 100))`.
- **All lines share the same stroke style** — there is no per-line colour/thickness. This is a major limitation for features 1-3.
- The `ui_control` method renders an inline stroke editor (`ui.add(&mut self.stroke)`) and a "Clear Painting" button — but these are rendered *inside* the canvas panel, not connected to the footer.

### Footer (`footer.rs`)
- Renders 3 tool buttons, 10 palette colour buttons, and 1 eraser button.
- **All button clicks just `println!()`** — nothing is connected to canvas state.
- Tool buttons use emoji labels: pen, fountain pen, pencil.
- Palette buttons call `self.palette.get_color(i)` for their fill colour.

### Palette (`palette.rs`)
- 10 individual `Color32` fields (`color0` through `color9`) — not an array.
- `get_color(i: u8) -> Option<Color32>` with a 10-arm match statement.
- Could be simplified to `[Color32; 10]` but works for now.

### Header (`header.rs`)
- Pure placeholder: 6 labels and a "test" label on the right.
- Uses `egui::MenuBar::new().ui()` with left-to-right and right-to-left layouts.

### Center widget (`center_widget.rs`)
- Measures content width on one frame, applies centering margin on the next.
- Used by footer to horizontally centre the toolbar.

## Communication patterns (or lack thereof)

**There is no shared state.** Each component owns its own state independently:
- `Snap` holds `header`, `canvas`, `footer` as sibling fields.
- `App::update` renders them in sequence but passes no shared context between them.
- Footer buttons print to stdout instead of modifying any shared state.

This is the **primary architectural gap** — we need a mechanism for footer tool/colour selection to flow into canvas drawing behaviour.

## Key egui APIs needed for the 11 features

### Feature 1: Colours (wire palette to canvas)
- Already have `Color32` in palette and `Stroke` in canvas
- Need: shared state for selected colour, apply to `canvas.stroke.color`

### Feature 2: Eraser
- `egui::Stroke` can use `Color32::TRANSPARENT` or we can remove points
- Better approach: track eraser as a tool mode; on drag, remove lines that intersect the eraser path
- Or simpler: draw with background colour (but this doesn't work on exported images cleanly)
- Best: store an eraser flag per-stroke, render eraser strokes with blend mode or skip them

### Feature 3: Line thickness
- `egui::Stroke::width` is already an `f32`
- `egui::Slider::new(value, range)` for thickness control
- Or use preset thickness buttons in the footer

### Feature 4: Shape selection (select + move shapes)
- `egui::Sense::click_and_drag()` for selection
- Need hit-testing against stored shapes
- `Rect::contains(pos)` for bounding box checks
- Transform selected shape's points by drag delta

### Feature 5: Undo/Redo
- Command pattern: store operations as a stack
- Or simpler: snapshot `lines` vec on each stroke completion
- `Vec<Vec<Vec<Pos2>>>` as history stack with cursor index
- egui has no built-in undo system

### Feature 6: Shape tools (rectangle, ellipse, arrow, line)
- `egui::Shape::rect_stroke(rect, corner_radius, stroke)` — rectangles
- `egui::Shape::ellipse_stroke(center, radius, stroke)` — ellipses
- `egui::Shape::line_segment([p1, p2], stroke)` — straight lines
- `egui::Shape::Path` for arrows (line + arrowhead)
- `Painter::add(shape)` to render them
- Need to extend data model beyond `Vec<Vec<Pos2>>` to support typed shapes

### Feature 7: Screenshot capture
- **`xcap` crate** (v0.8): `xcap::Monitor::all()` lists monitors, `monitor.capture_image()` returns `image::RgbaImage`
- Need to convert `RgbaImage` -> `egui::ColorImage` -> `egui::TextureHandle` for display
- `egui::ColorImage::from_rgba_unmultiplied(size, &pixels)` for conversion
- `ctx.load_texture(name, image, options)` to create a texture
- `ui.image(&texture)` or `Painter::image()` to render
- **`global-hotkey` crate** (v0.7): register system-wide keyboard shortcuts
  - `GlobalHotKeyManager::new()` — create manager
  - `HotKey::new(modifiers, code)` — define a hotkey
  - `manager.register(hotkey)` — register it
  - `GlobalHotKeyEvent::receiver()` — poll for events
  - Must be created/polled from the main thread
  - Can poll in `App::update` each frame

### Feature 8: Dark/light theme
- **`dark-light` crate** (v2.0): `dark_light::detect()` returns `Mode::Dark | Mode::Light | Mode::Default`
- egui has `Visuals::dark()` and `Visuals::light()` built-in
- Apply via `ctx.set_visuals(Visuals::dark())` or `ctx.set_visuals(Visuals::light())`
- Can detect once at startup and set accordingly
- Optionally re-detect periodically or on focus

### Feature 9: Export to PNG
- **`image` crate** (v0.25): `image::RgbaImage::save("path.png")`
- Need to render canvas to an offscreen buffer or capture the viewport
- `eframe::Frame` doesn't expose framebuffer directly
- Options:
  1. Re-render all shapes to an `image::RgbaImage` manually (most reliable)
  2. Use native file dialog via `rfd` crate (not yet a dependency)
- For file picker: `rfd::FileDialog::new().add_filter("PNG", &["png"]).save_file()`
- **Will need to add `rfd` as a dependency** for native file dialogs

### Feature 10: Text annotations
- `egui::TextEdit::singleline(&mut text)` or `multiline` for input
- `Painter::text(pos, anchor, text, font_id, color)` for rendering text on canvas
- Need a text tool mode: click to place, type text, press Enter to confirm
- Store as a `TextAnnotation { position: Pos2, text: String, font_size: f32, color: Color32 }`

### Feature 11: Clipboard copy/paste
- `egui::Context` has no built-in image clipboard
- Need platform-specific clipboard: `arboard` crate
  - `arboard::Clipboard::new()` → `clipboard.set_image(ImageData { width, height, bytes })`
  - `clipboard.get_image()` for paste
- **Will need to add `arboard` as a dependency**
- Keyboard shortcuts: `ctx.input(|i| i.key_pressed(Key::C) && i.modifiers.command)` for Ctrl+C/Cmd+C

## Recommended approach for shared state

### Introduce an `AppState` struct

The most natural pattern for egui is a single shared state struct passed by mutable reference to all components:

```rust
pub struct AppState {
    // Tool selection
    pub active_tool: Tool,

    // Drawing style
    pub stroke_colour: Color32,
    pub stroke_width: f32,

    // Canvas data (moved from Canvas)
    pub strokes: Vec<StrokeData>,
    pub undo_stack: Vec<Vec<StrokeData>>,
    pub redo_stack: Vec<Vec<StrokeData>>,

    // Theme
    pub theme: Theme,

    // Screenshot
    pub screenshot: Option<egui::TextureHandle>,
}

pub enum Tool {
    Pen,
    Highlighter,
    Pencil,
    Eraser,
    Rectangle,
    Ellipse,
    Arrow,
    Line,
    Text,
    Select,
}

pub struct StrokeData {
    pub kind: StrokeKind,
    pub colour: Color32,
    pub width: f32,
}

pub enum StrokeKind {
    Freehand(Vec<Pos2>),
    Rectangle(Rect),
    Ellipse { center: Pos2, radius: Vec2 },
    Arrow { from: Pos2, to: Pos2 },
    Line { from: Pos2, to: Pos2 },
    Text { position: Pos2, text: String, font_size: f32 },
}
```

### Threading model
- `App::update` runs on the main thread every frame
- Pass `&mut AppState` to each component's `render` method
- Footer modifies `active_tool` and `stroke_colour`
- Canvas reads `active_tool` to decide behaviour, reads `stroke_colour`/`stroke_width` for new strokes
- No multi-threading needed for UI state

### Trait changes
Change `View::render` signature:
```rust
pub trait View {
    fn render(&mut self, ui: &mut egui::Ui, state: &mut AppState);
}
```

This is a **breaking change** to all components but is a small, mechanical refactor.

## Architectural changes needed before features

### 1. Introduce `AppState` (prerequisite for everything)
- Create `state.rs` with shared `AppState`, `Tool`, `StrokeData`, `StrokeKind`
- Update `View` trait to accept `&mut AppState`
- Refactor `Snap::update` to create/pass `AppState`
- Move drawing data from `Canvas` into `AppState`

### 2. Refactor Canvas data model
- Replace `Vec<Vec<Pos2>>` with `Vec<StrokeData>` — each stroke stores its own colour, width, and shape kind
- This enables per-stroke colours, thickness, and multiple shape types
- Keep normalised coordinate system

### 3. Wire footer buttons to state
- Footer reads/writes `state.active_tool` and `state.stroke_colour`
- Show selected state visually (highlight active tool button, outline selected colour)

### 4. Add new dependencies (for later features)
- `rfd` — native file dialogs (export to PNG)
- `arboard` — clipboard image support (copy/paste)

### Recommended implementation order
The features have natural dependencies. Recommended batching:

**Batch 1 — Foundation (must be first)**
1. `AppState` + `View` trait refactor
2. Wire colours (feature 1)
3. Wire eraser (feature 2)
4. Line thickness (feature 3)

**Batch 2 — Core drawing**
5. Shape tools (feature 6)
6. Text annotations (feature 10)
7. Undo/Redo (feature 5)

**Batch 3 — Platform integration**
8. Dark/light theme (feature 8)
9. Screenshot capture (feature 7)
10. Export to PNG (feature 9)
11. Clipboard copy/paste (feature 11)

**Batch 4 — Advanced**
12. Shape selection/move (feature 4) — hardest, depends on shapes existing

## Notes on egui 0.33 specifics

- `egui::Stroke` implements `Widget` — can be added directly to UI as an editor (colour picker + width slider). Canvas already uses this in `ui_control`.
- `egui::color_picker::color_edit_button_srgba` — inline colour picker button
- `Frame::canvas(style)` — creates a dark background frame for drawing areas
- `Painter::extend(shapes)` — batch-add shapes (already used)
- `Painter::rect_stroke`, `Painter::circle_stroke`, `Painter::line_segment` — shape primitives
- `Painter::text` — text rendering with font selection
- `Response::drag_delta()` — pixel delta during drag
- `Response::drag_started()`, `drag_stopped()` — drag lifecycle
- `Sense::drag()`, `Sense::click()`, `Sense::click_and_drag()` — interaction types
- `ctx.input(|i| ...)` — read keyboard/mouse state
- `ctx.request_repaint()` — force a redraw (useful for animations)

## Risks and considerations

1. **Coordinate system**: Normalised coords work for freehand but rectangles/ellipses need careful handling during resize. Store shapes in normalised coords too for consistency.
2. **Performance**: With many strokes, rebuilding all shapes every frame could get slow. Consider caching the shape list and only rebuilding on change (egui redraws every frame regardless, but shape construction can be cached).
3. **Eraser complexity**: True eraser (removing parts of strokes) is complex. Start with whole-stroke eraser (remove any stroke the eraser touches), upgrade later.
4. **Text input in immediate mode**: egui's `TextEdit` needs careful state management — the text being edited must persist between frames. Store it in `AppState`.
5. **Screenshot capture timing**: Need to hide the Snap window before capturing, then restore it. Use `ctx.send_viewport_cmd(ViewportCommand::Minimized(true))`, capture, then restore.
6. **Global hotkeys**: `global-hotkey` must register from the main thread. Poll `GlobalHotKeyEvent::receiver()` in `App::update` each frame.
