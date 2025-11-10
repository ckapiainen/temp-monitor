use crate::{Message, Screen};
use crate::app::styles;
use iced::widget::{button, column, container, pick_list, row, svg};
use iced::{Center, Element, Fill, Theme};

/// Render the app with header
pub fn with_header<'a>(
                         content: Element<'a, Message>,
                         _current_screen: &Screen,
) -> Element<'a, Message> {
    let main_page_button = button(
        container(
            svg(svg::Handle::from_path("assets/icons/menu.svg"))
                .width(30)
                .height(30)
        )
        .align_x(Center)
        .align_y(Center)
        .width(35)
        .height(35)
    )
        .on_press(Message::MainButtonPressed)
        .style(styles::rounded_button_style);

    let plotter_page = button(
        container(
            svg(svg::Handle::from_path("assets/icons/chart-spline.svg"))
                .width(30)
                .height(30)
        )
        .align_x(Center)
        .align_y(Center)
        .width(35)
        .height(35)
    )
        .on_press(Message::PlotterButtonPressed)
        .style(styles::rounded_button_style);

    let settings_page = button(
        container(
            svg(svg::Handle::from_path("assets/icons/settings.svg"))
                .width(30)
                .height(30)
        )
        .align_x(Center)
        .align_y(Center)
        .width(35)
        .height(35)
    )
        .on_press(Message::ShowSettingsModal)
        .style(styles::rounded_button_style);

    let header = container(
        row![main_page_button, plotter_page, settings_page]
            .align_y(Center)
            .spacing(8),
    )
        .padding(10)
        .align_x(Center)
        .align_y(Center)
        .style(styles::header_container_style)
        .width(250);

    // center the header horizontally at top
    let header_wrapper = container(header)
        .width(Fill)
        .center_x(Fill);

    container(column![header_wrapper, content].spacing(20))
        .width(Fill)
        .height(Fill)
        .into()
}