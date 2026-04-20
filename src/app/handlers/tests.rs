use super::{history, scroll, update};
use crate::app::model::{FocusPart, HistoryItem};
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

// ── SearchChanged handler ────────────────────────────────────────────────────

#[test]
fn search_changed_updates_query_and_clears_hover_and_keyboard() {
    let mut app = AppModel::default();
    app.history.push_back(text_item("hello", false));
    app.hovered_index = Some(0);
    app.hovered_focus = Some((0, FocusPart::Entry));
    app.keyboard_focus = Some((0, FocusPart::Pin));

    let _ = update(&mut app, Message::SearchChanged("he".into()));

    assert_eq!(app.search_query, "he");
    assert!(app.hovered_index.is_none());
    assert!(app.hovered_focus.is_none());
    assert!(app.keyboard_focus.is_none());
}

#[test]
fn search_changed_empty_string_clears_query() {
    let mut app = AppModel::default();
    app.search_query = "old".into();

    let _ = update(&mut app, Message::SearchChanged(String::new()));

    assert!(app.search_query.is_empty());
}

// ── RemoveHistory ────────────────────────────────────────────────────────────

#[test]
fn remove_history_removes_entry_at_index() {
    let mut app = AppModel::default();
    app.history.push_back(text_item("first", false));
    app.history.push_back(text_item("second", false));
    app.history.push_back(text_item("third", false));

    let _ = update(&mut app, Message::RemoveHistory(1));

    assert_eq!(app.history.len(), 2);
    assert_eq!(item_text(&app.history[0]), "first");
    assert_eq!(item_text(&app.history[1]), "third");
}

#[test]
fn remove_history_last_item_leaves_empty() {
    let mut app = AppModel::default();
    app.history.push_back(text_item("only", false));

    let _ = update(&mut app, Message::RemoveHistory(0));

    assert!(app.history.is_empty());
}

// ── HoverEntry ───────────────────────────────────────────────────────────────

#[test]
fn hover_entry_sets_hover_state_and_clears_keyboard_focus() {
    let mut app = AppModel::default();
    app.history.push_back(text_item("item", false));
    app.keyboard_focus = Some((0, FocusPart::Entry));

    let _ = update(&mut app, Message::HoverEntry(Some((0, FocusPart::Pin))));

    assert_eq!(app.hovered_index, Some(0));
    assert_eq!(app.hovered_focus, Some((0, FocusPart::Pin)));
    assert!(app.keyboard_focus.is_none());
}

#[test]
fn hover_entry_none_clears_hover_state() {
    let mut app = AppModel::default();
    app.hovered_index = Some(2);
    app.hovered_focus = Some((2, FocusPart::Remove));

    let _ = update(&mut app, Message::HoverEntry(None));

    assert!(app.hovered_index.is_none());
    assert!(app.hovered_focus.is_none());
}

// ── Keyboard nav with filtered results ───────────────────────────────────────

#[test]
fn move_selection_down_steps_through_filtered_indices() {
    let mut app = AppModel::default();
    app.history.push_back(text_item("apple", false)); // idx 0 – matches
    app.history.push_back(text_item("banana", false)); // idx 1 – no match
    app.history.push_back(text_item("apricot", false)); // idx 2 – matches
    app.search_query = "ap".into();

    // First press: no previous selection → picks first filtered idx (0)
    let _ = update(&mut app, Message::MoveSelectionDown);
    assert_eq!(app.hovered_index, Some(0));

    // Second press: from idx 0 → next in [0,2] is idx 2
    let _ = update(&mut app, Message::MoveSelectionDown);
    assert_eq!(app.hovered_index, Some(2));

    // Third press: wraps back to first (idx 0)
    let _ = update(&mut app, Message::MoveSelectionDown);
    assert_eq!(app.hovered_index, Some(0));
}

#[test]
fn move_selection_up_wraps_to_last_filtered_index() {
    let mut app = AppModel::default();
    app.history.push_back(text_item("apple", false)); // idx 0
    app.history.push_back(text_item("banana", false)); // idx 1
    app.history.push_back(text_item("apricot", false)); // idx 2
    app.search_query = "ap".into();

    // No selection → Up picks last filtered (idx 2)
    let _ = update(&mut app, Message::MoveSelectionUp);
    assert_eq!(app.hovered_index, Some(2));

    // Again: from idx 2 → prev in [0,2] is idx 0
    let _ = update(&mut app, Message::MoveSelectionUp);
    assert_eq!(app.hovered_index, Some(0));
}

#[test]
fn move_selection_does_nothing_when_filtered_list_is_empty() {
    let mut app = AppModel::default();
    app.history.push_back(text_item("apple", false));
    app.search_query = "zzz".into();

    let _ = update(&mut app, Message::MoveSelectionDown);
    assert!(app.hovered_index.is_none());

    let _ = update(&mut app, Message::MoveSelectionUp);
    assert!(app.hovered_index.is_none());
}

// ── MoveFocusLeft / MoveFocusRight ───────────────────────────────────────────

#[test]
fn move_focus_right_cycles_entry_pin_remove() {
    let mut app = AppModel::default();
    app.history.push_back(text_item("item", false));
    app.hovered_index = Some(0);
    app.keyboard_focus = Some((0, FocusPart::Entry));

    let _ = update(&mut app, Message::MoveFocusRight);
    assert_eq!(app.keyboard_focus, Some((0, FocusPart::Pin)));

    let _ = update(&mut app, Message::MoveFocusRight);
    assert_eq!(app.keyboard_focus, Some((0, FocusPart::Remove)));

    let _ = update(&mut app, Message::MoveFocusRight);
    assert_eq!(app.keyboard_focus, Some((0, FocusPart::Entry)));
}

#[test]
fn move_focus_left_cycles_entry_remove_pin() {
    let mut app = AppModel::default();
    app.history.push_back(text_item("item", false));
    app.hovered_index = Some(0);
    app.keyboard_focus = Some((0, FocusPart::Entry));

    let _ = update(&mut app, Message::MoveFocusLeft);
    assert_eq!(app.keyboard_focus, Some((0, FocusPart::Remove)));

    let _ = update(&mut app, Message::MoveFocusLeft);
    assert_eq!(app.keyboard_focus, Some((0, FocusPart::Pin)));

    let _ = update(&mut app, Message::MoveFocusLeft);
    assert_eq!(app.keyboard_focus, Some((0, FocusPart::Entry)));
}

#[test]
fn move_focus_without_hover_initialises_to_entry() {
    let mut app = AppModel::default();
    app.history.push_back(text_item("item", false));
    app.hovered_index = Some(0);
    // keyboard_focus starts as None

    let _ = update(&mut app, Message::MoveFocusRight);
    assert_eq!(app.keyboard_focus, Some((0, FocusPart::Entry)));
}

// ── PopupClosed ──────────────────────────────────────────────────────────────

#[test]
fn popup_closed_clears_popup_and_search() {
    let mut app = AppModel::default();
    let id = cosmic::iced::window::Id::unique();
    app.popup = Some(id);
    app.popup_is_layer_surface = true;
    app.search_query = "hello".into();
    app.hovered_index = Some(1);
    app.at_scroll_bottom = true;

    let _ = update(&mut app, Message::PopupClosed(id));

    assert!(app.popup.is_none());
    assert!(!app.popup_is_layer_surface);
    assert!(app.search_query.is_empty());
    assert!(app.hovered_index.is_none());
    assert!(!app.at_scroll_bottom);
}

#[test]
fn popup_closed_ignores_mismatched_id() {
    let mut app = AppModel::default();
    let real_id = cosmic::iced::window::Id::unique();
    let other_id = cosmic::iced::window::Id::unique();
    app.popup = Some(real_id);
    app.search_query = "query".into();

    let _ = update(&mut app, Message::PopupClosed(other_id));

    assert_eq!(app.popup, Some(real_id));
    assert_eq!(app.search_query, "query");
}

// ── WindowUnfocused ──────────────────────────────────────────────────────────

#[test]
fn window_unfocused_only_closes_layer_surface_popups() {
    let mut app = AppModel::default();
    let id = cosmic::iced::window::Id::unique();
    app.popup = Some(id);
    app.popup_is_layer_surface = false; // XDG popup, should NOT be closed
    app.search_query = "query".into();

    let _ = update(&mut app, Message::WindowUnfocused(id));

    // Popup should remain open for non-layer-surface popups
    assert!(app.popup.is_some());
    assert_eq!(app.search_query, "query");
}

// ── IPC signal file path ─────────────────────────────────────────────────────

#[test]
fn ipc_signal_path_is_none_when_env_var_unset() {
    // Temporarily remove XDG_RUNTIME_DIR if present
    let saved = std::env::var("XDG_RUNTIME_DIR").ok();
    unsafe { std::env::remove_var("XDG_RUNTIME_DIR") };

    let result = crate::ipc::get_signal_file_path();
    assert!(result.is_none());

    // Restore
    if let Some(val) = saved {
        unsafe { std::env::set_var("XDG_RUNTIME_DIR", val) };
    }
}

#[test]
fn ipc_signal_path_appends_filename_to_runtime_dir() {
    unsafe { std::env::set_var("XDG_RUNTIME_DIR", "/tmp/test-runtime") };

    let result = crate::ipc::get_signal_file_path();

    // Restore – don't leave test pollution
    unsafe { std::env::remove_var("XDG_RUNTIME_DIR") };

    let path = result.expect("should return a path");
    assert_eq!(
        path.file_name().and_then(|n| n.to_str()),
        Some("clippy-land-toggle")
    );
    assert!(path.to_string_lossy().starts_with("/tmp/test-runtime"));
}
