use std::path::PathBuf;

pub(super) fn parse_first_local_path_from_uri_list(uri_list: &str) -> Option<PathBuf> {
    for line in uri_list.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if let Some(path) = path_from_file_uri(trimmed) {
            return Some(path);
        }
    }
    None
}

fn path_from_file_uri(uri: &str) -> Option<PathBuf> {
    let without_scheme = uri.strip_prefix("file://")?;
    let path_part = without_scheme.split(['\r', '\n']).next()?.trim();
    if path_part.is_empty() {
        return None;
    }
    percent_decode_to_path(path_part)
}

fn percent_decode_to_path(input: &str) -> Option<PathBuf> {
    let mut out = Vec::with_capacity(input.len());
    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'%' if i + 2 < bytes.len() => {
                let h = hex_val(bytes[i + 1])?;
                let l = hex_val(bytes[i + 2])?;
                out.push((h << 4) | l);
                i += 3;
            }
            b => {
                out.push(b);
                i += 1;
            }
        }
    }
    let decoded = String::from_utf8(out).ok()?;
    Some(PathBuf::from(decoded))
}

fn hex_val(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(10 + b - b'a'),
        b'A'..=b'F' => Some(10 + b - b'A'),
        _ => None,
    }
}
