use super::{ClipboardEntry, MAX_IMAGE_BYTES, THUMBNAIL_SIZE_PX, debug_log};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io::Cursor;
use std::path::Path;

pub(super) fn clipboard_entry_from_image_bytes(
    mime: String,
    bytes: Vec<u8>,
) -> Option<ClipboardEntry> {
    if bytes.is_empty() || bytes.len() > MAX_IMAGE_BYTES {
        return None;
    }

    let mut hasher = DefaultHasher::new();
    mime.hash(&mut hasher);
    bytes.hash(&mut hasher);
    let hash = hasher.finish();
    let thumbnail_png = make_thumbnail_png(&mime, &bytes);

    Some(ClipboardEntry::Image {
        mime,
        bytes,
        hash,
        thumbnail_png,
    })
}

pub(super) fn clipboard_entry_from_image_path(path: &Path) -> Option<ClipboardEntry> {
    let ext = path.extension()?.to_string_lossy().to_ascii_lowercase();
    let mime = match ext.as_str() {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "webp" => "image/webp",
        _ => return None,
    };

    let bytes = std::fs::read(path).ok()?;
    clipboard_entry_from_image_bytes(mime.to_string(), bytes)
}

fn make_thumbnail_png(mime: &str, bytes: &[u8]) -> Option<Vec<u8>> {
    let format = match mime {
        "image/png" => image::ImageFormat::Png,
        "image/jpeg" => image::ImageFormat::Jpeg,
        "image/webp" => image::ImageFormat::WebP,
        _ => {
            return image::load_from_memory(bytes)
                .ok()
                .and_then(encode_thumbnail_png);
        }
    };

    let decoded = image::load_from_memory_with_format(bytes, format)
        .or_else(|_| image::load_from_memory(bytes))
        .ok()?;

    encode_thumbnail_png(decoded)
}

fn encode_thumbnail_png(decoded: image::DynamicImage) -> Option<Vec<u8>> {
    let thumb = decoded.thumbnail(THUMBNAIL_SIZE_PX, THUMBNAIL_SIZE_PX);
    let mut out = Vec::new();
    let mut cursor = Cursor::new(&mut out);
    thumb.write_to(&mut cursor, image::ImageFormat::Png).ok()?;
    Some(out)
}

pub(super) fn log_image_too_large(len: usize) {
    debug_log(format!(
        "clipboard image ignored (too large): {} bytes (max {})",
        len, MAX_IMAGE_BYTES
    ));
}
