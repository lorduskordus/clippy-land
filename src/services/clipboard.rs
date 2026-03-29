mod image;
mod io;
mod model;
mod uri;

#[cfg(test)]
mod tests;

pub use model::{ClipboardEntry, ClipboardFingerprint};

const MAX_IMAGE_BYTES: usize = 8 * 1024 * 1024;
const THUMBNAIL_SIZE_PX: u32 = 400;

pub fn read_clipboard_entry() -> Option<ClipboardEntry> {
    io::read_clipboard_entry()
}

pub fn read_clipboard_text() -> Option<String> {
    io::read_clipboard_text()
}

pub fn read_clipboard_image() -> Option<ClipboardEntry> {
    io::read_clipboard_image()
}

pub fn write_clipboard_text(text: &str) -> bool {
    io::write_clipboard_text(text)
}

pub fn write_clipboard_image(mime: &str, bytes: &[u8]) -> bool {
    io::write_clipboard_image(mime, bytes)
}

pub(super) fn debug_log(message: impl std::fmt::Display) {
    if std::env::var_os("CLIPPY_LAND_DEBUG_CLIPBOARD").is_some() {
        eprintln!("[clippy-land] {message}");
    }
}
