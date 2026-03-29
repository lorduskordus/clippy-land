use crate::app::Message;
use cosmic::iced::{Background, Color, Vector};
use cosmic::prelude::*;
use cosmic::widget;

pub(super) fn highlight_history_target<'a>(
    content: Element<'a, Message>,
    active: bool,
) -> Element<'a, Message> {
    if !active {
        return content;
    }

    widget::container(content)
        .class(cosmic::theme::Container::custom(|theme| {
            let cosmic = theme.cosmic();
            cosmic::widget::container::Style {
                background: Some(cosmic::iced::Background::Color(
                    cosmic.background.component.hover.into(),
                )),
                border: cosmic::iced::Border {
                    radius: cosmic.corner_radii.radius_s.into(),
                    ..Default::default()
                },
                ..Default::default()
            }
        }))
        .into()
}

pub(super) fn transparent_entry_button_style(
    theme: &cosmic::Theme,
) -> cosmic::widget::button::Style {
    let cosmic = theme.cosmic();

    cosmic::widget::button::Style {
        shadow_offset: Vector::default(),
        background: Some(Background::Color(Color::TRANSPARENT)),
        overlay: None,
        border_radius: cosmic.corner_radii.radius_s.into(),
        border_width: 0.0,
        border_color: Color::TRANSPARENT,
        outline_width: 0.0,
        outline_color: Color::TRANSPARENT,
        icon_color: Some(cosmic.background.component.on.into()),
        text_color: Some(cosmic.background.component.on.into()),
    }
}
