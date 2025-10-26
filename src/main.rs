mod app;
mod collectors;

use iced::widget::{button, center, column, container, pick_list, row, text};
use iced::window::Position::Centered;
use iced::{window, Background, Center, Element, Fill, Task, Theme};
use app::{layout ,main_window};

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
        let page = match self.current_screen {
            Screen::Main => main_window::view(),
            Screen::Plotter => container("").into(),
            Screen::Settings => container("").into(),
        };


        layout::with_header(page, &self.current_theme, &self.current_screen)
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        window::close_events().map(Message::WindowClosed)
    }
}
