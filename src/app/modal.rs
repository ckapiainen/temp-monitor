use crate::Message;
use iced::widget::{button, center, column, container, mouse_area, opaque, stack, text};
use iced::{Color, Element};

/// Generic modal with a semi-transparent background and centered content
fn modal<'a, Message: Clone>(
    base: impl Into<Element<'a, Message>>,
    content: impl Into<Element<'a, Message>>,
    hide_modal: Message,
    close_with_background_click: bool,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let backdrop = mouse_area(center(opaque(content)).style(|_theme| {
        container::Style {
            background: Some(
                Color {
                    a: 0.7,
                    ..Color::BLACK
                }
                .into(),
            ),
            ..container::Style::default()
        }
    }));

    let backdrop = if close_with_background_click {
        backdrop.on_press(hide_modal)
    } else {
        backdrop
    };

    stack![base.into(), backdrop].into()
}
pub fn settings_view<'a>(
    base: impl Into<Element<'a, Message>>,
    show_modal: bool,
) -> Element<'a, Message> {
    if !show_modal {
        return base.into();
    }

    // Settings modal content
    let modal_content = container(column![
        text("Settings").size(24),
        button(text("Exit")).on_press(Message::HideSettingsModal),
    ])
    .padding(20)
    .style(container::rounded_box);

    modal(base, modal_content, Message::HideSettingsModal, false)
}
