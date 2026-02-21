use egui::{ColorImage, Context, Pos2, TextureOptions, Vec2};
use xcap::Monitor;

use crate::state::{AppState, DrawObject};

/// Captures the primary monitor and adds the image to the canvas.
pub fn capture_and_load(ctx: &Context, state: &mut AppState) {
    let image = match capture_primary_monitor() {
        Ok(img) => img,
        Err(e) => {
            eprintln!("Screenshot capture failed: {e}");
            return;
        }
    };

    let size = [image.width() as usize, image.height() as usize];
    let pixels: Vec<egui::Color32> = image
        .pixels()
        .map(|p| egui::Color32::from_rgba_premultiplied(p[0], p[1], p[2], p[3]))
        .collect();
    let color_image = ColorImage::new(size, pixels);

    let texture = ctx.load_texture("screenshot", color_image, TextureOptions::LINEAR);

    let aspect = size[0] as f32 / size[1] as f32;
    // Size the image to fill the canvas width in normalised coordinates (0..1).
    let img_size = Vec2::new(1.0, 1.0 / aspect);

    state.objects.push(DrawObject::Image {
        texture,
        pos: Pos2::ZERO,
        size: img_size,
    });
}

/// Captures the primary monitor, falling back to the first available.
fn capture_primary_monitor() -> Result<image::RgbaImage, String> {
    let monitors = Monitor::all().map_err(|e| format!("Failed to enumerate monitors: {e}"))?;

    let monitor = monitors
        .iter()
        .find(|m| m.is_primary().unwrap_or(false))
        .or(monitors.first())
        .ok_or_else(|| "No monitors found".to_string())?;

    monitor
        .capture_image()
        .map_err(|e| format!("Failed to capture monitor: {e}"))
}
