mod app;
mod collectors;

use std::time::Duration;
use iced::widget::container;
use iced::{window, Element, Subscription, Task, Theme};
use sysinfo::System;
use app::{layout, main_window};
use crate::collectors::cpu_collector::CpuData;
use crate::collectors::frequency_collector::FrequencyMonitor;

fn main() -> iced::Result {
    iced::daemon(|| App::new(), App::update, App::view)
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
    UpdateTemperatures,
}
#[derive(Clone, Debug)]
enum Screen {
    Main,
    Plotter,
    Settings,
}

struct App {
    window_id: Option<window::Id>,
    cpu_data: CpuData,
    system: System,
    current_screen: Screen,
    app_screen: Screen,
    current_theme: Theme,
    frequency_monitor: Option<FrequencyMonitor>, 
}
impl App {
    fn new() -> (Self, Task<Message>) {
        let window_settings = window::Settings {
            size: iced::Size::new(850.0, 600.0),
            position: window::Position::Centered,
            min_size: Some(iced::Size::new(500.0, 400.0)),
            resizable: true,
            decorations: true,
            level: window::Level::Normal,
            ..Default::default()
        };

        let (_, open_task) = window::open(window_settings);

        let mut system = System::new_all();
        system.refresh_cpu_all();

        let cpu_data = CpuData::new(&system);

        // Initialize frequency monitor
        let frequency_monitor = FrequencyMonitor::new(*cpu_data.get_base_frequency())
            .ok(); // If it fails just use base frequency

        (
            Self {
                window_id: None,
                cpu_data,
                system,
                current_screen: Screen::Main,
                app_screen: Screen::Main,
                current_theme: Theme::GruvboxDark,
                frequency_monitor,
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
            Message::UpdateTemperatures => {
                // Refresh system data and update CPU data
                self.system.refresh_cpu_all();
                self.cpu_data = CpuData::new(&self.system);

                // Update current frequency if monitor is available
                if let Some(ref monitor) = self.frequency_monitor {
                    if let Ok(freq) = monitor.get_current_frequency() {
                        self.cpu_data.update_frequency(freq);
                    }
                }

                Task::none()
            }
        }
    }

    fn view(&self, window_id: window::Id) -> Element<Message> {
        if self.window_id != Some(window_id) {
            return container("").into();
        }
        let page = match self.current_screen {
            Screen::Main => main_window::view(&self.cpu_data),
            Screen::Plotter => container("").into(),
            Screen::Settings => container("").into(),
        };


        layout::with_header(page, &self.current_theme, &self.current_screen)
    }

    fn subscription(&self) -> Subscription<Message> { // https://docs.iced.rs/iced/#passive-subscriptions
        Subscription::batch(vec![
            window::close_events().map(Message::WindowClosed),
            iced::time::every(Duration::from_secs(2)).map(|_| Message::UpdateTemperatures),
        ])
    }
}