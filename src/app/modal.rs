use iced::widget::{center, container, mouse_area, opaque, stack};
use iced::{Color, Element};

/// Generic modal with a semi-transparent background and centered content
pub fn modal<'a, Message>(
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
            }),
    );

    let backdrop = if close_with_background_click {
        opaque(background_layer.on_press(hide_modal))
    } else {
        opaque(background_layer)
    };

    stack![base.into(), backdrop].into()
}
