use super::row::history_row;
use crate::app::{AppModel, Message, icons};
use crate::fl;
use crate::services::clipboard::ClipboardEntry;
use cosmic::iced::{Alignment, Length, window::Id};
use cosmic::prelude::*;
use cosmic::widget;

pub(super) fn view(app: &AppModel) -> Element<'_, Message> {
    app.core
        .applet
        .icon_button("edit-copy-symbolic")
        .on_press(Message::TogglePopup)
        .into()
}

/// Returns the indices into `app.history` that match the current search query.
pub(crate) fn filtered_indices(app: &AppModel) -> Vec<usize> {
    let query = app.search_query.to_lowercase();
    if query.is_empty() {
        return (0..app.history.len()).collect();
    }
    app.history
        .iter()
        .enumerate()
        .filter(|(_, item)| match &item.entry {
            ClipboardEntry::Text(text) => text.to_lowercase().contains(&query),
            ClipboardEntry::Image { mime, .. } => mime.to_lowercase().contains(&query),
        })
        .map(|(idx, _)| idx)
        .collect()
}

pub(super) fn view_window(app: &AppModel, _id: Id) -> Element<'_, Message> {
    let visible = filtered_indices(app);
    let mut history_column = widget::column::Column::new().spacing(4);

    if app.history.is_empty() {
        history_column = history_column.push(
            widget::container(widget::text::body(fl!("empty")))
                .width(Length::Fill)
                .center_x(Length::Fill),
        );
    } else if visible.is_empty() {
        history_column = history_column.push(
            widget::container(widget::text::body(fl!("no-results")))
                .width(Length::Fill)
                .center_x(Length::Fill),
        );
    } else {
        let pinned_count = app.history.iter().filter(|it| it.pinned).count();

        for &idx in &visible {
            // Show divider between pinned and unpinned sections when not filtering
            if app.search_query.is_empty()
                && idx == pinned_count
                && pinned_count > 0
                && pinned_count < app.history.len()
            {
                history_column = history_column.push(widget::divider::horizontal::default());
            }

            history_column = history_column.push(history_row(app, idx, &app.history[idx]));
        }
    }

    let history_scrollable = widget::container(
        widget::scrollable(
            widget::container(history_column)
                .padding([0, 12, 0, 12])
                .width(Length::Fill),
        )
        .id(crate::app::history_scroll_id())
        .on_scroll(Message::HistoryScrolled)
        .width(Length::Fill),
    )
    .max_height(400.0)
    .width(Length::Fill);

    let search_bar = widget::container(
        widget::search_input(fl!("search-placeholder"), &app.search_query)
            .on_input(Message::SearchChanged)
            .on_clear(Message::SearchChanged(String::new()))
            .width(Length::Fill),
    )
    .padding([0, 12]);

    let mut content = widget::column::Column::new().spacing(8).padding([8, 8]);

    if !app.history.is_empty() {
        content = content.push(search_bar);
    }

    content = content.push(history_scrollable);

    if !app.history.is_empty() && app.search_query.is_empty() {
        let delete_all_button = widget::button::destructive(fl!("delete-all"))
            .leading_icon(icons::remove_icon())
            .on_press(Message::ClearHistory);

        let controls_sheet = widget::container(delete_all_button)
            .padding([8, 8])
            .align_x(Alignment::End)
            .width(Length::Fill);
        content = content.push(controls_sheet);
    }

    app.core.applet.popup_container(content).into()
}
