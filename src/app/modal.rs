use crate::app::styles;
use crate::{model, Message};
use iced::widget::{
    button, center, column, container, mouse_area, opaque, pick_list, row, rule, stack, text,
    text_input,
};
use iced::{Alignment, Color, Element, Length, Theme};


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
            }),
    );

    let backdrop = if close_with_background_click {
        opaque(background_layer.on_press(hide_modal))
    } else {
        opaque(background_layer)
    };

    stack![base.into(), backdrop].into()
}


// TODO: More settings
// Update interval picker (0.5s, 1s, 2s, 5s)
// Startup Behavior:
// "Start with Windows" checkbox
// "Start minimized to tray" checkbox
// Temperature Units
// Tray icon settings:
// "Show temperature" checkbox
// "Show CPU usage" checkbox
// "Show power draw" checkbox

pub fn settings_view<'a>(base: Element<'a, Message>, settings: &'a model::config::Settings) -> Element<'a, Message> {
    // Header with title and close button
    let header = container(
        row![
            text("Settings")
                .size(24)
                .width(Length::Fill)
                .style(|_theme| text::Style {
                    color: Some(Color::from_rgb(0.9, 0.9, 0.9))
                }),
            button(text("✕").size(20))
                .on_press(Message::HideSettingsModal)
                .padding([4, 10])
                .style(styles::header_button_style),
        ]
            .align_y(Alignment::Center)
            .spacing(10),
    )
        .padding([15, 20])
        .width(Length::Fill);

    // Theme picker
    let theme_section = column![
        text("Theme")
            .size(16)
            .style(|_theme| text::Style {
                color: Some(Color::from_rgb(0.8, 0.8, 0.8))
            }),
        pick_list(
            [Theme::Dracula, Theme::Ferra, Theme::Dark, Theme::Nord],
            Some(&settings.theme),
            Message::ThemeChanged,
        )
        .width(Length::Fill)
        .padding(10),
    ]
        .spacing(8);

    // Temperature threshold inputs
    let temp_section = column![
        text("Temperature Thresholds")
            .size(16)
            .style(|_theme| text::Style {
                color: Some(Color::from_rgb(0.8, 0.8, 0.8))
            }),
        text("Configure temperature ranges for tray icon color changes")
            .size(12)
            .style(|_theme| text::Style {
                color: Some(Color::from_rgb(0.6, 0.6, 0.6))
            }),
        row![
            column![
                text("Low Threshold (°C)")
                    .size(14)
                    .style(|_theme| text::Style {
                        color: Some(Color::from_rgb(0.7, 0.7, 0.7))
                    }),
                text_input("60", &settings.temp_low_input)
                    .on_input(Message::TempLowThresholdChanged)
                    .padding(10)
                    .width(Length::Fixed(80.0)),
            ]
            .spacing(5),
            column![
                text("High Threshold (°C)")
                    .size(14)
                    .style(|_theme| text::Style {
                        color: Some(Color::from_rgb(0.7, 0.7, 0.7))
                    }),
                text_input("80", &settings.temp_high_input)
                    .on_input(Message::TempHighThresholdChanged)
                    .padding(10)
                    .width(Length::Fixed(80.0)),
            ]
            .spacing(5),
        ]
        .spacing(15),
        text("Low: ≤ Low threshold | Medium: Between thresholds | High: ≥ High threshold")
            .size(11)
            .style(|_theme| text::Style {
                color: Some(Color::from_rgb(0.55, 0.55, 0.55))
            }),
    ]
        .spacing(8);

    // Save button
    let save_button = button(
        text("Save Settings")
            .width(Length::Fill)
            .align_x(iced::alignment::Horizontal::Center),
    )
        .on_press(Message::SaveSettings)
        .padding(12)
        .width(Length::Fill)
        .style(styles::rounded_button_style);

    // Combine all sections
    let content = column![
        header,
        rule::horizontal(1),
        container(
            column![theme_section, temp_section, save_button]
                .spacing(20)
                .padding([20, 0]),
        )
        .padding([10, 20])
        .width(Length::Fill)
        .height(Length::Fill),
    ]
        .width(450)
        .height(500);

    // Modal content container
    let modal_content = container(content)
        .width(450)
        .height(500)
        .style(styles::modal_generic);

    modal(base, modal_content, Message::HideSettingsModal, false)
}