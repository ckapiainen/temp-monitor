use crate::app::styles;
use crate::{model, Message};
use iced::widget::{
    button, center, checkbox, column, container, mouse_area, opaque, pick_list, row, rule,
    scrollable, slider, stack, text, text_input,
};
use iced::{Alignment, Color, Element, Length, Theme};

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

// TODO: More settings
// Update interval picker (0.5s, 1s, 2s, 5s)
// Tray icon settings:
// "Show temperature" checkbox
// "Show CPU usage" checkbox
// "Show power draw" checkbox

pub fn settings_view<'a>(
    base: Element<'a, Message>,
    settings: &'a model::config::Settings,
) -> Element<'a, Message> {
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

    // ========== APPEARANCE SECTION ==========
    let appearance_section = column![
        text("APPEARANCE").size(14).style(|_theme| text::Style {
            color: Some(Color::from_rgb(0.6, 0.6, 0.6))
        }),
        text("Theme").size(15).style(|_theme| text::Style {
            color: Some(Color::from_rgb(0.9, 0.9, 0.9))
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

    // ========== BEHAVIOR SECTION ==========
    let behavior_section = column![
        text("BEHAVIOR").size(14).style(|_theme| text::Style {
            color: Some(Color::from_rgb(0.6, 0.6, 0.6))
        }),
        checkbox("Start with Windows", settings.start_with_windows)
            .on_toggle(Message::ToggleStartWithWindows),
        checkbox("Start minimized to tray", settings.start_minimized)
            .on_toggle(Message::ToggleStartMinimized),
        column![
            text("Update Interval")
                .size(15)
                .style(|_theme| text::Style {
                    color: Some(Color::from_rgb(0.9, 0.9, 0.9))
                }),
            row![
                slider(
                    0.5..=10.0,
                    settings.data_update_interval,
                    Message::UpdateIntervalChanged
                )
                .step(0.5)
                .width(Length::Fill),
                container(
                    text(format!("{:.1}s", settings.data_update_interval))
                        .size(14)
                        .style(|_theme| text::Style {
                            color: Some(Color::from_rgb(0.8, 0.8, 0.8))
                        })
                )
                .width(Length::Fixed(50.0))
                .align_x(iced::alignment::Horizontal::Right),
            ]
            .spacing(10)
            .align_y(Alignment::Center),
            text("How often to refresh hardware data")
                .size(12)
                .style(|_theme| text::Style {
                    color: Some(Color::from_rgb(0.6, 0.6, 0.6))
                }),
        ]
        .spacing(5),
    ]
    .spacing(8);

    // ========== TEMPERATURE SECTION ==========
    let unit = settings.selected_temp_units.map(|u| match u {
        model::config::TempUnits::Celsius => "°C",
        model::config::TempUnits::Fahrenheit => "°F",
    });

    let temp_section = column![
        text("TEMPERATURE").size(14).style(|_theme| text::Style {
            color: Some(Color::from_rgb(0.6, 0.6, 0.6))
        }),
        column![
            text("Units").size(15).style(|_theme| text::Style {
                color: Some(Color::from_rgb(0.9, 0.9, 0.9))
            }),
            pick_list(
                [
                    model::config::TempUnits::Celsius,
                    model::config::TempUnits::Fahrenheit,
                ],
                settings.selected_temp_units,
                Message::TempUnitSelected,
            )
            .width(140)
            .padding(10),
        ]
        .spacing(5),
        column![
            text("Thresholds").size(15).style(|_theme| text::Style {
                color: Some(Color::from_rgb(0.9, 0.9, 0.9))
            }),
            row![
                column![
                    text(format!("Low ({})", unit.unwrap_or("°C")))
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
                    text(format!("High ({})", unit.unwrap_or("°C")))
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
            text("Configure temperature ranges for tray icon color changes")
                .size(12)
                .style(|_theme| text::Style {
                    color: Some(Color::from_rgb(0.6, 0.6, 0.6))
                }),
            text("Low: ≤ Low threshold | Medium: Between thresholds | High: ≥ High threshold")
                .size(11)
                .style(|_theme| text::Style {
                    color: Some(Color::from_rgb(0.55, 0.55, 0.55))
                }),
        ]
        .spacing(5),
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
    let separator_color = Color::from_rgb(0.3, 0.3, 0.3);

    let scrollbar_config = scrollable::Scrollbar::new().scroller_width(4);
    let content = column![
        header,
        rule::horizontal(1),
        container(
            scrollable(
                container(
                    column![
                        appearance_section,
                        rule::horizontal(1).style(move |_theme| rule::Style {
                            color: separator_color,
                            snap: false,
                            fill_mode: rule::FillMode::Full,
                            radius: 0.0.into(),
                        }),
                        behavior_section,
                        rule::horizontal(1).style(move |_theme| rule::Style {
                            color: separator_color,
                            snap: false,
                            fill_mode: rule::FillMode::Full,
                            radius: 0.0.into(),
                        }),
                        temp_section,
                        rule::horizontal(1).style(move |_theme| rule::Style {
                            color: separator_color,
                            snap: false,
                            fill_mode: rule::FillMode::Full,
                            radius: 0.0.into(),
                        }),
                        save_button,
                    ]
                    .spacing(10)
                )
                .padding(20)
                .width(Length::Fill),
            )
            .direction(scrollable::Direction::Vertical(scrollbar_config))
            .style(styles::thin_scrollbar_style)
        )
        .padding(20)
        .width(Length::Fill)
        .height(Length::Fill),
    ]
    .width(Length::Fill)
    .height(Length::Fill);

    // Modal content container
    let modal_content = container(content)
        .width(500)
        .height(600)
        .style(styles::modal_generic);

    modal(base, modal_content, Message::HideSettingsModal, false)
}
