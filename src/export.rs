use std::path::Path;

use egui::{Color32, Pos2};
use image::{ImageBuffer, Rgba, RgbaImage};

use crate::state::DrawObject;

/// Renders all draw objects to a PNG file at the given path.
///
/// Objects are stored in normalised 0..1 coordinates (with square proportions).
/// We map them into a `width x height` pixel buffer, matching egui's
/// `RectTransform::from_to(Rect(0..proportions), Rect(0..size))` logic.
pub fn export_png(
    objects: &[DrawObject],
    width: u32,
    height: u32,
    background: Color32,
    path: &Path,
) -> Result<(), String> {
    let mut img: RgbaImage = ImageBuffer::from_pixel(
        width,
        height,
        Rgba([
            background.r(),
            background.g(),
            background.b(),
            background.a(),
        ]),
    );

    // Compute the same square-proportions scaling that canvas.rs uses.
    let (fw, fh) = square_proportions(width as f32, height as f32);

    for obj in objects {
        render_object(&mut img, obj, width as f32, height as f32, fw, fh);
    }

    img.save(path)
        .map_err(|e| format!("Failed to save PNG: {e}"))
}

/// Mirrors `egui::Vec2::square_proportions` — returns (px, py) where the
/// shorter side is 1.0 and the longer side is > 1.0.
fn square_proportions(w: f32, h: f32) -> (f32, f32) {
    if w >= h {
        (w / h, 1.0)
    } else {
        (1.0, h / w)
    }
}

/// Converts a normalised coordinate to pixel space, matching egui's RectTransform.
fn to_pixel(p: Pos2, img_w: f32, img_h: f32, prop_w: f32, prop_h: f32) -> (f32, f32) {
    let x = p.x / prop_w * img_w;
    let y = p.y / prop_h * img_h;
    (x, y)
}

fn render_object(
    img: &mut RgbaImage,
    obj: &DrawObject,
    img_w: f32,
    img_h: f32,
    prop_w: f32,
    prop_h: f32,
) {
    match obj {
        DrawObject::Freehand {
            points,
            colour,
            width,
        } => {
            if points.len() < 2 {
                return;
            }
            let rgba = colour_to_rgba(*colour);
            let half = (*width / 2.0).max(0.5);
            for pair in points.windows(2) {
                let (x0, y0) = to_pixel(pair[0], img_w, img_h, prop_w, prop_h);
                let (x1, y1) = to_pixel(pair[1], img_w, img_h, prop_w, prop_h);
                draw_thick_line(img, x0, y0, x1, y1, half, rgba);
            }
        }
        DrawObject::Rectangle {
            min,
            max,
            colour,
            width,
        } => {
            let (x0, y0) = to_pixel(*min, img_w, img_h, prop_w, prop_h);
            let (x1, y1) = to_pixel(*max, img_w, img_h, prop_w, prop_h);
            let rgba = colour_to_rgba(*colour);
            let half = (*width / 2.0).max(0.5);
            // Four edges
            draw_thick_line(img, x0, y0, x1, y0, half, rgba);
            draw_thick_line(img, x1, y0, x1, y1, half, rgba);
            draw_thick_line(img, x1, y1, x0, y1, half, rgba);
            draw_thick_line(img, x0, y1, x0, y0, half, rgba);
        }
        DrawObject::Ellipse {
            center,
            radius_x,
            radius_y,
            colour,
            width,
        } => {
            let (cx, cy) = to_pixel(*center, img_w, img_h, prop_w, prop_h);
            let sx = img_w / prop_w;
            let sy = img_h / prop_h;
            let rx = radius_x * sx;
            let ry = radius_y * sy;
            let rgba = colour_to_rgba(*colour);
            let half = (*width / 2.0).max(0.5);
            draw_ellipse_outline(img, cx, cy, rx, ry, half, rgba);
        }
        DrawObject::Line {
            start,
            end,
            colour,
            width,
        } => {
            let (x0, y0) = to_pixel(*start, img_w, img_h, prop_w, prop_h);
            let (x1, y1) = to_pixel(*end, img_w, img_h, prop_w, prop_h);
            let rgba = colour_to_rgba(*colour);
            let half = (*width / 2.0).max(0.5);
            draw_thick_line(img, x0, y0, x1, y1, half, rgba);
        }
        DrawObject::Arrow {
            start,
            end,
            colour,
            width,
        } => {
            let (ax, ay) = to_pixel(*start, img_w, img_h, prop_w, prop_h);
            let (bx, by) = to_pixel(*end, img_w, img_h, prop_w, prop_h);
            let rgba = colour_to_rgba(*colour);
            let half = (*width / 2.0).max(0.5);
            draw_thick_line(img, ax, ay, bx, by, half, rgba);

            // Arrowhead — mirrors canvas.rs logic
            let dx = bx - ax;
            let dy = by - ay;
            let len = (dx * dx + dy * dy).sqrt();
            if len > 0.0 {
                let dir_x = dx / len;
                let dir_y = dy / len;
                let perp_x = -dir_y;
                let perp_y = dir_x;
                let arrow_len = 10.0_f32;
                let tip1_x = bx - arrow_len * dir_x + arrow_len * 0.4 * perp_x;
                let tip1_y = by - arrow_len * dir_y + arrow_len * 0.4 * perp_y;
                let tip2_x = bx - arrow_len * dir_x - arrow_len * 0.4 * perp_x;
                let tip2_y = by - arrow_len * dir_y - arrow_len * 0.4 * perp_y;
                draw_thick_line(img, bx, by, tip1_x, tip1_y, half, rgba);
                draw_thick_line(img, bx, by, tip2_x, tip2_y, half, rgba);
            }
        }
        // Text and Image rendering are not yet fully supported; skip silently.
        DrawObject::Text { .. } | DrawObject::Image { .. } => {}
    }
}

fn colour_to_rgba(c: Color32) -> Rgba<u8> {
    Rgba([c.r(), c.g(), c.b(), c.a()])
}

/// Draws a line with thickness using Bresenham, filling a circle of `half_w` at each step.
fn draw_thick_line(
    img: &mut RgbaImage,
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
    half_w: f32,
    colour: Rgba<u8>,
) {
    let dx = x1 - x0;
    let dy = y1 - y0;
    let dist = (dx * dx + dy * dy).sqrt();
    let steps = (dist * 2.0).max(1.0) as u32;

    for i in 0..=steps {
        let t = i as f32 / steps as f32;
        let px = x0 + dx * t;
        let py = y0 + dy * t;
        fill_circle(img, px, py, half_w, colour);
    }
}

/// Fills a circle of given radius at (cx, cy) with the given colour, alpha-blending.
fn fill_circle(img: &mut RgbaImage, cx: f32, cy: f32, radius: f32, colour: Rgba<u8>) {
    let (w, h) = img.dimensions();
    let r_ceil = radius.ceil() as i32;
    let cx_i = cx as i32;
    let cy_i = cy as i32;
    let r_sq = radius * radius;

    for dy in -r_ceil..=r_ceil {
        for dx in -r_ceil..=r_ceil {
            if (dx * dx + dy * dy) as f32 <= r_sq {
                let px = cx_i + dx;
                let py = cy_i + dy;
                if px >= 0 && py >= 0 && (px as u32) < w && (py as u32) < h {
                    blend_pixel(img, px as u32, py as u32, colour);
                }
            }
        }
    }
}

/// Alpha-blends `src` over the existing pixel at (x, y).
fn blend_pixel(img: &mut RgbaImage, x: u32, y: u32, src: Rgba<u8>) {
    let dst = img.get_pixel(x, y);
    let sa = src[3] as f32 / 255.0;
    let da = dst[3] as f32 / 255.0;
    let out_a = sa + da * (1.0 - sa);
    if out_a == 0.0 {
        return;
    }
    let blend =
        |s: u8, d: u8| -> u8 { ((s as f32 * sa + d as f32 * da * (1.0 - sa)) / out_a) as u8 };
    *img.get_pixel_mut(x, y) = Rgba([
        blend(src[0], dst[0]),
        blend(src[1], dst[1]),
        blend(src[2], dst[2]),
        (out_a * 255.0) as u8,
    ]);
}

/// Draws an ellipse outline by sampling points around the perimeter.
fn draw_ellipse_outline(
    img: &mut RgbaImage,
    cx: f32,
    cy: f32,
    rx: f32,
    ry: f32,
    half_w: f32,
    colour: Rgba<u8>,
) {
    let circumference =
        std::f32::consts::PI * (3.0 * (rx + ry) - ((3.0 * rx + ry) * (rx + 3.0 * ry)).sqrt());
    let steps = (circumference * 2.0).max(64.0) as u32;

    let mut prev_x = cx + rx;
    let mut prev_y = cy;

    for i in 1..=steps {
        let angle = 2.0 * std::f32::consts::PI * i as f32 / steps as f32;
        let cur_x = cx + rx * angle.cos();
        let cur_y = cy + ry * angle.sin();
        draw_thick_line(img, prev_x, prev_y, cur_x, cur_y, half_w, colour);
        prev_x = cur_x;
        prev_y = cur_y;
    }
}

/// Opens a native file-save dialog and exports the canvas to PNG.
/// Returns `Ok(Some(path))` if saved, `Ok(None)` if the user cancelled.
pub fn export_with_dialog(
    objects: &[DrawObject],
    canvas_width: u32,
    canvas_height: u32,
    background: Color32,
) -> Result<Option<std::path::PathBuf>, String> {
    let path = rfd::FileDialog::new()
        .add_filter("PNG image", &["png"])
        .set_file_name("snap-export.png")
        .save_file();

    match path {
        Some(p) => {
            export_png(objects, canvas_width, canvas_height, background, &p)?;
            Ok(Some(p))
        }
        None => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exports_empty_canvas() {
        let dir = std::env::temp_dir();
        let path = dir.join("snap_test_empty.png");
        let result = export_png(&[], 100, 100, Color32::WHITE, &path);
        assert!(result.is_ok());
        assert!(path.exists());
        let img = image::open(&path).unwrap().to_rgba8();
        assert_eq!(img.dimensions(), (100, 100));
        // All pixels should be white
        assert_eq!(img.get_pixel(0, 0), &Rgba([255, 255, 255, 255]));
        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn exports_freehand_stroke() {
        let objects = vec![DrawObject::Freehand {
            points: vec![Pos2::new(0.0, 0.5), Pos2::new(1.0, 0.5)],
            colour: Color32::RED,
            width: 2.0,
        }];
        let dir = std::env::temp_dir();
        let path = dir.join("snap_test_freehand.png");
        let result = export_png(&objects, 200, 200, Color32::WHITE, &path);
        assert!(result.is_ok());
        let img = image::open(&path).unwrap().to_rgba8();
        // The midpoint of a horizontal line at y=0.5 should have red pixels
        let mid = img.get_pixel(100, 100);
        assert_eq!(mid[0], 255); // red channel
        assert_eq!(mid[1], 0);
        assert_eq!(mid[2], 0);
        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn square_proportions_landscape() {
        let (pw, ph) = square_proportions(1600.0, 900.0);
        assert!((pw - 1600.0 / 900.0).abs() < 0.001);
        assert!((ph - 1.0).abs() < 0.001);
    }

    #[test]
    fn square_proportions_portrait() {
        let (pw, ph) = square_proportions(600.0, 800.0);
        assert!((pw - 1.0).abs() < 0.001);
        assert!((ph - 800.0 / 600.0).abs() < 0.001);
    }
}
