use arboard::{Clipboard, ImageData};

/// Copies RGBA image data to the system clipboard.
pub fn copy_to_clipboard(image_data: &[u8], width: usize, height: usize) {
    let Ok(mut clipboard) = Clipboard::new() else {
        #[cfg(debug_assertions)]
        eprintln!("clipboard: failed to open clipboard");
        return;
    };

    let img = ImageData {
        width,
        height,
        bytes: std::borrow::Cow::Borrowed(image_data),
    };

    if let Err(_err) = clipboard.set_image(img) {
        #[cfg(debug_assertions)]
        eprintln!("clipboard: failed to set image: {_err}");
    }
}

/// Reads an image from the system clipboard, returning RGBA bytes, width, and height.
pub fn paste_from_clipboard() -> Option<(Vec<u8>, usize, usize)> {
    let mut clipboard = Clipboard::new().ok()?;
    let img = clipboard.get_image().ok()?;
    Some((img.bytes.into_owned(), img.width, img.height))
}
