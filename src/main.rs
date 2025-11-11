#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide terminal on Windows
mod app;
mod collectors;
mod model;

use crate::collectors::cpu_collector::CpuData;
use crate::collectors::lhm_collector::lhm_cpu_queries;
use crate::collectors::CoreStats;
use app::{layout, main_window, modal};
use colored::Colorize;
use iced::widget::container;
use iced::{window, Element, Subscription, Task, Theme};
use lhm_client::service::is_service_installed;
use lhm_client::{ComputerOptions, LHMClient};
use std::time::Duration;
use sysinfo::System;
use tray_icon::{
    menu::{Menu, MenuEvent, MenuId, MenuItem, PredefinedMenuItem},
    Icon, TrayIconBuilder,
};
use model::config::Settings;

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
    TrayEvent(MenuId),
    ShowSettingsModal,
    HideSettingsModal,
    ThemeChanged(Theme),
    TempLowThresholdChanged(String),
    TempHighThresholdChanged(String),
    SaveSettings,
    MainButtonPressed,
    PlotterButtonPressed,
    UpdateHardwareData,
    CpuValuesUpdated((f32, f32, Vec<CoreStats>)),
    MainWindow(main_window::Message),
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
    show_modal: bool,
    current_theme: Theme,
    settings: Settings,
    main_window: main_window::MainWindow,
    tray_icon: tray_icon::TrayIcon,
    show_menu_id: MenuId,
    quit_menu_id: MenuId,
}

impl App {
    /// Update tray tooltip with live hw data
    // Temperature thresholds for icon color changes are configurable in settings
    fn update_tray_tooltip(&self) {
        let tooltip = format!(
            "CPU: {:.0}°C ({:.0}%)\nPower: {:.1}W",
            self.cpu_data.temp, self.cpu_data.usage, self.cpu_data.total_power_draw
        );

        if let Err(e) = self.tray_icon.set_tooltip(Some(&tooltip)) {
            eprintln!("Failed to update tray tooltip: {}", e);
        }
    }

    fn new() -> (Self, Task<Message>) {
        let window_settings = window::Settings {
            size: iced::Size::new(800.0, 700.0),
            position: window::Position::Centered,
            min_size: Some(iced::Size::new(500.0, 400.0)),
            icon: window::icon::from_file("assets/logo.ico").ok(),
            resizable: true,
            decorations: true,
            level: window::Level::Normal,
            ..Default::default()
        };

        let (_, open_task) = window::open(window_settings);

        // Load tray icon from bytes
        const ICON_DATA: &[u8] = include_bytes!("../assets/logo.ico");
        let image = image::load_from_memory(ICON_DATA)
            .expect("Failed to load icon from memory")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).expect("Failed to create icon");
        // Create tray menu
        let menu = Menu::new();
        let show_item = MenuItem::new("Show Window", true, None);
        let quit_item = MenuItem::new("Quit", true, None);
        let separator = PredefinedMenuItem::separator();

        // Store menu IDs for event handling
        let show_id = show_item.id().clone();
        let quit_id = quit_item.id().clone();

        menu.append_items(&[&show_item, &separator, &quit_item])
            .expect("Failed to append menu items");

        // Build tray icon
        let tray_icon = TrayIconBuilder::new()
            .with_tooltip("TempMon")
            .with_icon(icon)
            .with_menu(Box::new(menu))
            .build()
            .expect("Failed to create tray icon");

        let mut system = System::new_all();
        system.refresh_cpu_all();
        let cpu_data = CpuData::new(&system);
        let hw_monitor_service = None;
        let settings = Settings::load().expect("Error loading settings");
        let current_theme = settings.theme.clone();

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
                show_modal: false,
                current_theme,
                settings,
                main_window: main_window::MainWindow::new(),
                tray_icon,
                show_menu_id: show_id,
                quit_menu_id: quit_id,
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
                    // Trigger initial update after service connects
                    Task::done(Message::UpdateHardwareData)
                } else {
                    Task::none()
                }
            }
            Message::WindowOpened(id) => {
                self.window_id = Some(id);
                Task::none()
            }
            Message::WindowClosed(id) => {
                dbg!("Window closed, daemon still running...");
                self.window_id = None;
                Task::none()
            }
            Message::TrayEvent(menu_id) => {
                if menu_id == self.show_menu_id {
                    // If window is closed, reopen it
                    if self.window_id.is_none() {
                        let window_settings = window::Settings {
                            size: iced::Size::new(800.0, 700.0),
                            position: window::Position::Centered,
                            min_size: Some(iced::Size::new(500.0, 400.0)),
                            icon: window::icon::from_file("assets/logo.ico").ok(),
                            resizable: true,
                            decorations: true,
                            level: window::Level::Normal,
                            ..Default::default()
                        };
                        let (_, open_task) = window::open(window_settings);
                        return open_task.map(Message::WindowOpened);
                    }
                    Task::none()
                } else if menu_id == self.quit_menu_id {
                    std::process::exit(0);
                } else {
                    Task::none()
                }
            }
            Message::ThemeChanged(theme) => {
                self.settings.theme = theme.clone();
                Task::none()
            }
            Message::TempLowThresholdChanged(value) => {
                self.settings.temp_low_input = value;
                Task::none()
            }
            Message::TempHighThresholdChanged(value) => {
                self.settings.temp_high_input = value;
                Task::none()
            }
            Message::SaveSettings => {
                // Parse and validate temperature thresholds
                if let Ok(low) = self.settings.temp_low_input.parse::<f32>() {
                    if let Ok(high) = self.settings.temp_high_input.parse::<f32>() {
                        if low < high {
                            self.settings.temp_low_threshold = low;
                            self.settings.temp_high_threshold = high;
                            self.current_theme = self.settings.theme.clone();
                        }
                    }
                }
                Settings::save(&self.settings);
                self.show_modal = false;
                Task::none()
            }
            Message::MainButtonPressed | Message::PlotterButtonPressed => {
                println!("Button pressed");
                Task::none()
            }
            Message::ShowSettingsModal => {
                // Reset input fields to current saved values when opening modal
                self.settings.temp_low_input = self.settings.temp_low_threshold.to_string();
                self.settings.temp_high_input = self.settings.temp_high_threshold.to_string();
                self.show_modal = true;
                Task::none()
            }
            Message::HideSettingsModal => {
                self.show_modal = false;
                Task::none()
            }
            Message::MainWindow(msg) => {
                self.main_window.update(msg);
                Task::none()
            }
            Message::UpdateHardwareData => {
                self.cpu_data.update(&mut self.system);

                if let Some(client) = &self.hw_monitor_service {
                    let client = client.clone();
                    Task::future(async move {
                        // NOTE TO SELF: Task::future always needs to return message
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
                self.cpu_data.update_lhm_data(temps);
                // Update tray tooltip with fresh hardware data
                self.update_tray_tooltip();

                Task::none()
            }
        }
    }

    fn view(&self, window_id: window::Id) -> Element<Message> {
        if self.window_id != Some(window_id) {
            return container("").into();
        }
        let page = match self.current_screen {
            Screen::Main => self
                .main_window
                .view(&self.cpu_data)
                .map(Message::MainWindow),
            Screen::Plotter => container("").into(),
            Screen::Settings => container("").into(),
        };
        if self.show_modal {
            modal::settings_view(layout::with_header(page), &self.settings)
        } else {
            layout::with_header(page)
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        // https://docs.iced.rs/iced/#passive-subscriptions
        Subscription::batch(vec![
            window::close_events().map(Message::WindowClosed),
            iced::time::every(Duration::from_secs(1)).map(|_| Message::UpdateHardwareData),
            tray_events_subscription(),
            self.main_window.subscription().map(Message::MainWindow),
        ])
    }
}

/// Subscription for tray menu events
fn tray_events_subscription() -> Subscription<Message> {
    use iced::futures::SinkExt;

    Subscription::run(|| {
        iced::stream::channel(
            50,
            |mut output: iced::futures::channel::mpsc::Sender<Message>| async move {
                loop {
                    tokio::time::sleep(Duration::from_millis(50)).await;

                    // Poll menu events from tray-icon
                    while let Ok(event) = MenuEvent::receiver().try_recv() {
                        let _ = output.send(Message::TrayEvent(event.id)).await;
                    }
                }
            },
        )
    })
}