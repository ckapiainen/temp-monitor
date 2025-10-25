use crate::collectors::cpu_collector::CpuData;
use crate::Message;
use iced::widget::{button, container, pick_list, row};
use iced::{window, Background, Center, Element, Fill, Theme};
use std::sync::mpsc::Receiver;
use std::time::Duration;
use sysinfo::System;

pub fn view(current_theme: &Theme) -> Element<'_, Message> {
    let main_page_button = button("Main")
        .on_press(Message::MainButtonPressed)
        .style(|_theme: &Theme, status| button::primary(&Theme::Dark, status));
    let plotter_page = button("Plotter")
        .on_press(Message::PlotterButtonPressed)
        .style(|_theme: &Theme, status| button::primary(&Theme::Dark, status));
    let settings_page = button("Settings")
        .on_press(Message::SettingsButtonPressed)
        .style(|_theme: &Theme, status| button::primary(&Theme::Dark, status));

    // Clone the Theme so the pick_list owns its selected value (avoids borrowing `current_theme`)
    let theme_picker = pick_list(
        Theme::ALL,
        Some(current_theme.clone()),
        Message::ThemeChanged,
    )
    .placeholder("Choose theme")
    .text_size(15)
    .width(150);

    let theme_section = container(
        row![main_page_button, plotter_page, settings_page, theme_picker]
            .align_y(Center)
            .spacing(10),
    )
    .align_x(Center)
    .align_y(Center)
    .style(|theme: &Theme| {
        let dracula_bg = Theme::Nord.extended_palette().background.base.color;

        container::Style {
            background: Some(Background::Color(dracula_bg)),
            border: iced::Border {
                color: iced::Color::from_rgba(0.1, 0.5, 0.5, 0.0),
                width: 1.0,
                radius: iced::border::Radius {
                    top_left: 0.0,
                    top_right: 0.0,
                    bottom_left: 10.0,
                    bottom_right: 10.0,
                },
            },
            ..Default::default()
        }
    })
    .width(450)
    .height(50);

    container(theme_section).width(Fill).center_x(Fill).into()
}
