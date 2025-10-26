use iced::widget::{button, container};
use iced::{Background, Theme};

/// Styling for components, currently only dark theme is supported

pub fn rounded_button_style(_theme: &Theme, status: button::Status) -> button::Style {
    match status {
        button::Status::Active => button::Style {
            background: Some(Background::Color(iced::Color::from_rgb(0.2, 0.2, 0.21))),
            border: iced::Border {
                color: iced::Color::from_rgba(0.35, 0.35, 0.4, 0.4),
                width: 1.5,
                radius: iced::border::Radius::from(12.0),
            },
            text_color: iced::Color::from_rgb(0.85, 0.85, 0.85),
            shadow: iced::Shadow {
                color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.3),
                offset: iced::Vector::new(0.0, 1.0),
                blur_radius: 3.0,
            },
            snap: false,
        },
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(iced::Color::from_rgb(0.24, 0.24, 0.26))),
            border: iced::Border {
                color: iced::Color::from_rgba(0.45, 0.45, 0.5, 0.6),
                width: 1.5,
                radius: iced::border::Radius::from(12.0),
            },
            text_color: iced::Color::WHITE,
            shadow: iced::Shadow {
                color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.4),
                offset: iced::Vector::new(0.0, 2.0),
                blur_radius: 6.0,
            },
            snap: false,
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(iced::Color::from_rgb(0.16, 0.16, 0.17))),
            border: iced::Border {
                color: iced::Color::from_rgba(0.3, 0.3, 0.35, 0.5),
                width: 1.5,
                radius: iced::border::Radius::from(12.0),
            },
            text_color: iced::Color::from_rgb(0.7, 0.7, 0.7),
            shadow: iced::Shadow {
                color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.2),
                offset: iced::Vector::new(0.0, 1.0),
                blur_radius: 2.0,
            },
            snap: false,
        },
        button::Status::Disabled => button::Style {
            background: Some(Background::Color(iced::Color::from_rgb(0.15, 0.15, 0.15))),
            border: iced::Border {
                color: iced::Color::from_rgba(0.2, 0.2, 0.2, 0.3),
                width: 1.5,
                radius: iced::border::Radius::from(12.0),
            },
            text_color: iced::Color::from_rgb(0.4, 0.4, 0.4),
            shadow: iced::Shadow::default(),
            snap: false,
        },
    }
}

pub fn card_container_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(iced::Color::from_rgb(0.18, 0.18, 0.19))),
        border: iced::Border {
            color: iced::Color::from_rgba(0.4, 0.4, 0.45, 0.5),
            width: 2.0,
            radius: iced::border::Radius::from(15.0),
        },
        shadow: iced::Shadow {
            color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.4),
            offset: iced::Vector::new(0.0, 2.0),
            blur_radius: 8.0,
        },
        text_color: None,
        snap: false,
    }
}

pub fn header_container_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(iced::Color::from_rgb(0.18, 0.18, 0.19))),
        border: iced::Border {
            color: iced::Color::from_rgba(0.4, 0.4, 0.45, 0.5),
            width: 2.0,
            radius: iced::border::Radius {
                top_left: 0.0,
                top_right: 0.0,
                bottom_left: 15.0,
                bottom_right: 15.0,
            },
        },
        shadow: iced::Shadow {
            color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.4),
            offset: iced::Vector::new(0.0, 2.0),
            blur_radius: 8.0,
        },
        text_color: None,
        snap: false,
    }
}
