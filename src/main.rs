mod app;
mod collectors;

use iced::widget::{button, center, column, container, pick_list, row, text};
use iced::window::Position::Centered;
use iced::{window, Background, Center, Element, Fill, Task, Theme};
use app::main_window;

fn main() -> iced::Result {
    iced::daemon(App::new, App::update, App::view)
        .subscription(App::subscription)
        .title("TempMon")
        .theme(App::theme)
        .run()
}

#[derive(Clone, Debug)]
enum Message {
    WindowOpened(window::Id),
    WindowClosed(window::Id),
    ThemeChanged(Theme),
    MainButtonPressed,
    PlotterButtonPressed,
    SettingsButtonPressed,
}
#[derive(Clone, Debug)]
enum Screen {
    Main,
    Plotter,
    Settings,
}

struct App {
    window_id: Option<window::Id>,
    current_screen: Screen,
    app_screen: Screen,
    current_theme: Theme,
}
impl App {
    fn new() -> (Self, Task<Message>) {
        let window_settings = window::Settings {
            size: iced::Size::new(800.0, 600.0),
            position: window::Position::Centered,
            min_size: Some(iced::Size::new(500.0, 400.0)),
            resizable: true,
            decorations: true,
            level: window::Level::Normal,
            ..Default::default()
        };

        let (_, open_task) = window::open(window_settings);

        (
            Self {
                window_id: None,
                current_screen: Screen::Main,
                app_screen: Screen::Main,
                current_theme: Theme::GruvboxDark,
            },
            open_task.map(Message::WindowOpened),
        )
    }

    fn theme(&self, _window: window::Id) -> Theme {
        self.current_theme.clone()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::WindowOpened(id) => {
                self.window_id = Some(id);
                Task::none()
            }
            Message::WindowClosed(_) => {
                println!("Window closed, daemon still running..."); // debug reminder...
                Task::none()
            }
            Message::ThemeChanged(theme) => {
                self.current_theme = theme;
                Task::none()
            }
            Message::MainButtonPressed
            | Message::PlotterButtonPressed
            | Message::SettingsButtonPressed => {
                println!("Button pressed");
                Task::none()
            }
        }
    }

    fn view(&self, window_id: window::Id) -> Element<Message> {
        if self.window_id != Some(window_id) {
            return container("").into();
        }
        match self.current_screen {
            Screen::Main => main_window::view(&self.current_theme),
            Screen::Plotter => container("").into(),
            Screen::Settings => container("").into(),
        }


        // let main_page_button = button("Main")
        //     .on_press(Message::MainButtonPressed)
        //     .style(|_theme: &Theme, status| button::primary(&Theme::Dark, status));
        // let plotter_page = button("Plotter")
        //     .on_press(Message::PlotterButtonPressed)
        //     .style(|_theme: &Theme, status| button::primary(&Theme::Dark, status));
        // let settings_page = button("Settings")
        //     .on_press(Message::SettingsButtonPressed)
        //     .style(|_theme: &Theme, status| button::primary(&Theme::Dark, status));
        //
        // let theme_picker = pick_list(Theme::ALL, Some(&self.current_theme), Message::ThemeChanged)
        //     .placeholder("Choose theme")
        //     .text_size(15)
        //     .width(150);
        //
        // let theme_section = container(
        //     row![main_page_button, plotter_page, settings_page, theme_picker]
        //         .align_y(Center)
        //         .spacing(10),
        // )
        //     .align_x(Center)
        //     .align_y(Center)
        //     .style(|theme: &Theme| {
        //         let dracula_bg = Theme::Nord.extended_palette().background.base.color;
        //
        //         container::Style {
        //             background: Some(Background::Color(dracula_bg)),
        //             border: iced::Border {
        //                 color: iced::Color::from_rgba(0.1, 0.5, 0.5, 0.0),
        //                 width: 1.0,
        //                 radius: iced::border::Radius {
        //                     top_left: 0.0,
        //                     top_right: 0.0,
        //                     bottom_left: 10.0,
        //                     bottom_right: 10.0,
        //                 },
        //             },
        //             ..Default::default()
        //         }
        //     })
        //     .width(450)
        //     .height(50);
        //
        // container(theme_section)
        //     .width(Fill)
        //     .center_x(Fill)
        //     .into()
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        window::close_events().map(Message::WindowClosed)
    }
}
