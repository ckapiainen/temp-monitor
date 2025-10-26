use crate::{Message, Screen};
use crate::app::styles;
use iced::widget::{button, column, container, pick_list, row};
use iced::{Center, Element, Fill, Theme};

/// Render the app with header
pub fn with_header<'a>(
                         content: Element<'a, Message>,
                         current_theme: &'a Theme,
                         current_screen: &Screen,
) -> Element<'a, Message> {
    let main_page_button = button("Main")
        .on_press(Message::MainButtonPressed)
        .padding([8, 16])
        .style(styles::rounded_button_style);

    let plotter_page = button("Plotter")
        .on_press(Message::PlotterButtonPressed)
        .padding([8, 16])
        .style(styles::rounded_button_style);

    let settings_page = button("Settings")
        .on_press(Message::SettingsButtonPressed)
        .padding([8, 16])
        .style(styles::rounded_button_style);

    let theme_picker = pick_list(
        Theme::ALL,
        Some(current_theme),
        Message::ThemeChanged,
    )
        .placeholder("Choose theme")
        .text_size(15)
        .width(150);

    let header = container(
        row![main_page_button, plotter_page, settings_page, theme_picker]
            .align_y(Center)
            .spacing(12),
    )
        .padding(10)
        .align_x(Center)
        .align_y(Center)
        .style(styles::header_container_style)
        .width(500);

    // center the header horizontally at top
    let header_wrapper = container(header)
        .width(Fill)
        .center_x(Fill);

    container(column![header_wrapper, content].spacing(20))
        .width(Fill)
        .height(Fill)
        .into()
}