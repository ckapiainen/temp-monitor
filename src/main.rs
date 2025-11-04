mod app;
mod collectors;

use crate::collectors::cpu_collector::CpuData;
use crate::collectors::lhm_collector::{lhm_cpu_queries, CoreStats};
use app::{layout, main_window};
use colored::Colorize;
use iced::widget::container;
use iced::{window, Element, Subscription, Task, Theme};
use lhm_client::service::is_service_installed;
use lhm_client::{ComputerOptions, LHMClient};
use std::time::Duration;
use sysinfo::System;

async fn connect_to_lhwm_service() -> Option<lhm_client::LHMClientHandle> {
    match LHMClient::connect().await {
        Ok(client) => {
            println!("Connected to hardware monitoring service");
            client
                .set_options(ComputerOptions {
                    controller_enabled: false,
                    cpu_enabled: true,
                    gpu_enabled: false,
                    motherboard_enabled: false,
                    battery_enabled: false,
                    memory_enabled: false,
                    network_enabled: false,
                    psu_enabled: true,
                    storage_enabled: false,
                })
                .await
                .unwrap();
            client.update_all().await.unwrap();
            println!("{}", "Service options set".green().bold());
            Some(client)
        }
        Err(e) => {
            eprintln!("{} {}", "Failed to connect to service: {}".red(), e);
            eprintln!("{}", "The service may not be running. Try:".red());
            eprintln!("{}", "1. Run 'install-service.bat' as administrator".red());
            eprintln!(
                "{}",
                "2. Or manually start the service from Services (services.msc)".red()
            );
            None
        }
    }
}

fn main() -> iced::Result {
    match is_service_installed() {
        Ok(true) => {
            println!("{}", "✓ Service is ready".green());
        }
        Ok(false) => {
            eprintln!(
                "{}",
                "Hardware monitoring service not installed".red().bold()
            );
            eprintln!(
                "{}",
                "Please run 'install-service.bat' as administrator"
                    .red()
                    .bold()
            );
            // TODO: Show user a dialog or instructions
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("Error checking service: {}", e);
            std::process::exit(1);
        }
    }

    iced::daemon(|| App::new(), App::update, App::view)
        .subscription(App::subscription)
        .title("TempMon")
        .theme(App::theme)
        .run()
}

#[derive(Clone)]
enum Message {
    WindowOpened(window::Id),
    WindowClosed(window::Id),
    ThemeChanged(Theme),
    MainButtonPressed,
    PlotterButtonPressed,
    SettingsButtonPressed,
    UpdateHardwareData,
    CpuValuesUpdated((f32, f32, Vec<CoreStats>)),
    HardwareMonitorConnected(Option<lhm_client::LHMClientHandle>),
}
#[derive(Clone, Debug)]
enum Screen {
    Main,
    Plotter,
    Settings,
}

struct App {
    window_id: Option<window::Id>,
    hw_monitor_service: Option<lhm_client::LHMClientHandle>,
    cpu_data: CpuData,
    system: System,
    current_screen: Screen,
    app_screen: Screen,
    current_theme: Theme,
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
        let hw_monitor_service = None;
        // Create task to connect to hardware monitor
        let connect_task = Task::future(async {
            Message::HardwareMonitorConnected(connect_to_lhwm_service().await)
        });
        (
            Self {
                window_id: None,
                hw_monitor_service,
                cpu_data,
                system,
                current_screen: Screen::Main,
                app_screen: Screen::Main,
                current_theme: Theme::GruvboxDark,
            },
            Task::batch(vec![
                // Batch tasks to run in parallel
                open_task.map(Message::WindowOpened),
                connect_task,
            ]),
        )
    }

    fn theme(&self, _window: window::Id) -> Theme {
        self.current_theme.clone()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::HardwareMonitorConnected(client) => {
                self.hw_monitor_service = client;
                if self.hw_monitor_service.is_some() {
                    println!("{}", "✓ Connected to hardware monitor".green());
                }
                Task::none()
            }
            Message::WindowOpened(id) => {
                self.window_id = Some(id);
                Task::none()
            }
            Message::WindowClosed(_) => {
                dbg!("Window closed, daemon still running...");
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
            Message::UpdateHardwareData => {
                self.cpu_data.update(&mut self.system);

                if let Some(client) = &self.hw_monitor_service {
                    let client = client.clone();
                    Task::future(async move { // NOTE TO SELF: Task::future always needs to return message
                        client.update_all().await.expect("Error updating hardware");
                        let temps = lhm_cpu_queries(&client).await;
                        Message::CpuValuesUpdated(temps)
                    })
                } else {
                    Task::none()
                }
            }
            Message::CpuValuesUpdated(temps) => {
                // Collect everything from lhm queries into CpuData
                self.cpu_data.cpu_temp = temps.0;
                self.cpu_data.total_power_draw = temps.1;
                self.cpu_data.core_power_draw = temps.2;
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

    fn subscription(&self) -> Subscription<Message> {
        // https://docs.iced.rs/iced/#passive-subscriptions
        Subscription::batch(vec![
            window::close_events().map(Message::WindowClosed),
            iced::time::every(Duration::from_secs(2)).map(|_| Message::UpdateHardwareData),
        ])
    }
}
