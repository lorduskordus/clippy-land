use super::{history, scroll, update};
use crate::app::model::HistoryItem;
use crate::app::{AppModel, Message};
use crate::services::clipboard;

fn text_entry(text: &str) -> clipboard::ClipboardEntry {
    clipboard::ClipboardEntry::Text(text.to_string())
}

fn text_item(text: &str, pinned: bool) -> HistoryItem {
    HistoryItem {
        entry: text_entry(text),
        pinned,
    }
}

fn item_text(item: &HistoryItem) -> &str {
    match &item.entry {
        clipboard::ClipboardEntry::Text(text) => text,
        clipboard::ClipboardEntry::Image { .. } => {
            panic!("expected text entry in handler tests")
        }
    }
}

#[test]
fn ignores_empty_and_short_numericish_entries() {
    assert!(history::should_ignore_clipboard_entry(""));
    assert!(history::should_ignore_clipboard_entry("  \n\t  "));
    assert!(history::should_ignore_clipboard_entry("12-34"));
    assert!(history::should_ignore_clipboard_entry("1,2,3"));
}

#[test]
fn keeps_nontrivial_entries() {
    assert!(!history::should_ignore_clipboard_entry("123456789"));
    assert!(!history::should_ignore_clipboard_entry("abc123"));
    assert!(!history::should_ignore_clipboard_entry("42 is the answer"));
}

#[test]
fn clipboard_changed_dedupes_and_preserves_pin_state() {
    let repeated = text_entry("repeat");
    let mut app = AppModel::default();
    app.history.push_back(text_item("front", false));
    app.history.push_back(HistoryItem {
        entry: repeated.clone(),
        pinned: true,
    });
    app.history.push_back(text_item("tail", false));

    let _ = update(&mut app, Message::ClipboardChanged(repeated.clone()));

    let matches = app.history.iter().filter(|it| it.entry == repeated).count();
    assert_eq!(matches, 1);

    let idx = app
        .history
        .iter()
        .position(|it| it.entry == repeated)
        .expect("entry should still exist");
    assert!(app.history[idx].pinned);
}

#[test]
fn toggling_pinned_item_moves_it_after_pinned_section() {
    let mut app = AppModel::default();
    app.history.push_back(text_item("a", true));
    app.history.push_back(text_item("b", true));
    app.history.push_back(text_item("c", false));

    let _ = update(&mut app, Message::TogglePin(0));

    assert!(app.history[0].pinned);
    assert_eq!(item_text(&app.history[0]), "b");
    assert!(!app.history[1].pinned);
    assert_eq!(item_text(&app.history[1]), "a");
}

#[test]
fn toggle_pin_respects_max_pinned_limit() {
    let mut app = AppModel::default();
    for i in 0..history::MAX_PINNED {
        app.history.push_back(text_item(&format!("pin-{i}"), true));
    }
    app.history.push_back(text_item("unpinned", false));

    let _ = update(&mut app, Message::TogglePin(history::MAX_PINNED));

    assert_eq!(history::pinned_count(&app.history), history::MAX_PINNED);
    assert_eq!(item_text(&app.history[history::MAX_PINNED]), "unpinned");
    assert!(!app.history[history::MAX_PINNED].pinned);
}

#[test]
fn clipboard_changed_trims_to_max_history() {
    let mut app = AppModel::default();
    for i in 0..history::MAX_HISTORY {
        app.history
            .push_back(text_item(&format!("item-{i}"), false));
    }

    let _ = update(
        &mut app,
        Message::ClipboardChanged(text_entry("fresh-entry")),
    );

    assert_eq!(app.history.len(), history::MAX_HISTORY);
    assert_eq!(
        item_text(app.history.front().expect("front entry exists")),
        "fresh-entry"
    );
    assert!(!app.history.iter().any(|it| item_text(it) == "item-29"));
}

#[test]
fn clear_history_removes_all_entries() {
    let mut app = AppModel::default();
    app.history.push_back(text_item("pinned", true));
    app.history.push_back(text_item("regular", false));

    let _ = update(&mut app, Message::ClearHistory);

    assert!(app.history.is_empty());
}

#[test]
fn clear_history_is_safe_for_empty_history() {
    let mut app = AppModel::default();

    let _ = update(&mut app, Message::ClearHistory);

    assert!(app.history.is_empty());
}

#[test]
fn desired_scroll_y_moves_selection_into_visible_window() {
    let offset = scroll::desired_scroll_y(Some(0.4), Some(0.25), 19, 30);

    assert!(matches!(offset, Some(value) if (value - 0.7).abs() < 0.000_1));
}

#[test]
fn desired_scroll_y_skips_when_selection_is_already_centered() {
    let offset = scroll::desired_scroll_y(Some(0.42857143), Some(0.3), 13, 30);

    assert_eq!(offset, None);
}

#[test]
fn desired_scroll_y_falls_back_to_target_ratio_without_viewport() {
    let offset = scroll::desired_scroll_y(None, None, 5, 10);

    assert_eq!(offset, Some(0.55));
}
