use crate::app::model::HistoryItem;
use crate::services::clipboard::{self, ClipboardEntry};
use std::collections::VecDeque;

pub(super) const MAX_HISTORY: usize = 30;
pub(super) const MAX_PINNED: usize = 5;

pub(super) fn pinned_count(history: &VecDeque<HistoryItem>) -> usize {
    history.iter().filter(|it| it.pinned).count()
}

pub(super) fn insert_after_pins(history: &mut VecDeque<HistoryItem>, item: HistoryItem) {
    let pos = history.iter().take_while(|it| it.pinned).count();
    history.insert(pos, item);
}

pub(super) fn trim_history(history: &mut VecDeque<HistoryItem>) {
    while history.len() > MAX_HISTORY {
        if let Some(idx) = history.iter().rposition(|it| !it.pinned) {
            let _ = history.remove(idx);
        } else {
            break;
        }
    }
}

pub(super) fn toggle_pin(history: &mut VecDeque<HistoryItem>, index: usize) {
    let Some(mut item) = history.remove(index) else {
        return;
    };

    if item.pinned {
        item.pinned = false;
        insert_after_pins(history, item);
    } else if pinned_count(history) >= MAX_PINNED {
        history.insert(index, item);
    } else {
        item.pinned = true;
        insert_after_pins(history, item);
    }
}

pub(super) fn copy_history_item(item: &HistoryItem) {
    copy_clipboard_entry(&item.entry);
}

pub(super) fn copy_clipboard_entry(entry: &ClipboardEntry) {
    match entry {
        ClipboardEntry::Text(text) => {
            _ = clipboard::write_clipboard_text(text);
        }
        ClipboardEntry::Image { mime, bytes, .. } => {
            _ = clipboard::write_clipboard_image(mime, bytes);
        }
    }
}

pub(super) fn should_ignore_clipboard_entry(entry: &str) -> bool {
    let trimmed = entry.trim();
    if trimmed.is_empty() {
        return true;
    }

    if trimmed.chars().all(|c| {
        c.is_ascii_digit() || matches!(c, ',' | '.' | ':' | ';' | '/' | '\\' | '_' | '-' | ' ')
    }) && trimmed.chars().count() <= 8
    {
        return true;
    }

    false
}
