use iced::widget::{button, column, container, pick_list, text};
use iced::{window, Center, Element, Task, Theme};

fn main() -> iced::Result {
    iced::daemon(App::new, App::update, App::view)
        .subscription(App::subscription)
        .title("TempMon")
        .theme(App::theme)
        .run()
}

struct App {
    window_id: Option<window::Id>,
    current_theme: Theme,  // Store selected theme
}

#[derive(Clone, Debug)]
enum Message {
    WindowOpened(window::Id),
    WindowClosed(window::Id),
    ThemeChanged(Theme),  // New message
}

impl App {
    fn new() -> (Self, Task<Message>) {
        let (_, open_task) = window::open(window::Settings::default());

        (
            Self {
                window_id: None,
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
                self.current_theme = theme;  // Update theme
                Task::none()
            }
        }
    }

    fn view(&self, window_id: window::Id) -> Element<Message> {
        if self.window_id != Some(window_id) {
            return container("").into();
        }

        // Theme picker dropdown
        let theme_picker = pick_list(
            Theme::ALL,
            Some(&self.current_theme),
            Message::ThemeChanged,
        )
            .placeholder("Choose theme");

        let content = column![
            text("Pick a Theme:").size(20),
            theme_picker,
            text(""),
        ]
            .spacing(20)
            .align_x(Center);

        container(content)
            .padding(40)
            .center_x(iced::Fill)
            .center_y(iced::Fill)
            .into()
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        window::close_events().map(Message::WindowClosed)
    }
}