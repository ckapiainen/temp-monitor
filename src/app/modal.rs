use crate::app::{layout, styles};
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
    let background_layer = mouse_area(
        container(center(opaque(content)))
            .width(iced::Fill)
            .height(iced::Fill)
            .style(|_theme| {
                // Background styling
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
            })
    );

    let backdrop = if close_with_background_click {
        opaque(background_layer.on_press(hide_modal))
    } else {
        opaque(background_layer)
    };

    stack![base.into(), backdrop].into()
}
pub fn settings_view(
    base: Element<Message>,
) -> Element<Message> {

    // Settings modal content
    let modal_content = container(column![
        text("Settings").size(24),
        button(text("Exit")).on_press(Message::HideSettingsModal),
    ])
    .padding(20)
    .width(350)
    .height(350)
    .style(styles::modal_generic);
    modal(base, modal_content, Message::HideSettingsModal, false)
}
