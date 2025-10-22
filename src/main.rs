#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
mod app;
mod collectors;

use std::process::exit;
use std::sync::{mpsc, Mutex};
use std::thread;
use std::time::Duration;
use app::ui_main_window::MainWindow;
use eframe::egui::{self, Context, Visuals};
use crate::collectors::cpu_collector::CpuData;
use sysinfo::System;
use tray_icon::{menu::{Menu, MenuItem, MenuEvent}, Icon, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{ShowWindow, SW_HIDE, SW_SHOWDEFAULT};
use winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};
static VISIBLE: Mutex<bool> = Mutex::new(true);
fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    // ----TRAY ICON CRAP -----
    let quit_i = MenuItem::with_id( "quit", "Quit", true, None);
    let icon = Icon::from_path("assets/favicon.ico", None)?;
    let tray_menu = Menu::new();
    tray_menu.append_items(&[&quit_i])?;
    let _tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_icon(icon)
        .with_tooltip("TempMon")
        .build()?;
    // ^^^^^ TRAY ICON CRAP ^^^^

    // ----NATIVE WINDOW SETUP -----
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([520.0, 340.0])
            .with_min_inner_size([300.0, 240.0])
            .with_icon(
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/img.png")[..])
                    .expect("Failed to load icon"),
            ),
        // .with_decorations(false) // Disable window decorations
        // .with_transparent(true), // Enable transparency
        ..Default::default()
    };
    eframe::run_native(
        "TempMon",
        options,
        Box::new(|cc| {
            let ctx = cc.egui_ctx.clone();
            set_styles(&cc.egui_ctx);

            // TRAY ICON https://github.com/emilk/egui/discussions/737
            let RawWindowHandle::Win32(handle) = cc.window_handle().unwrap().as_raw() else {
                panic!("Unsupported platform");
            };
            MenuEvent::set_event_handler(Some(move |event: MenuEvent| {
                if event.id == "quit" {
                    exit(0);
                }
            }));
            TrayIconEvent::set_event_handler(Some(move |event: TrayIconEvent| {

                match event {
                    TrayIconEvent::Click {
                        button_state: MouseButtonState::Down,
                        ..
                    } => {
                        let mut visible = VISIBLE.lock().unwrap();

                        if *visible {
                            let window_handle = HWND(handle.hwnd.into());
                            unsafe {
                                ShowWindow(window_handle, SW_HIDE);
                            }
                            *visible = false;
                        } else {
                            let window_handle = HWND(handle.hwnd.into());
                            unsafe {
                                ShowWindow(window_handle, SW_SHOWDEFAULT);
                            }
                            *visible = true;
                        }

                    }
                    _ => (),
                }
            }));

            // THREAD TO UPDATE CPU DATA
            let (tx, rx) = mpsc::channel();
            let mut sys = System::new_all();
            thread::spawn(move || {
                loop {
                    sys.refresh_cpu_all();
                    if tx.send(CpuData::new(&sys)).is_err() {
                        break; // Receiver dropped, exit thread
                    }
                    ctx.request_repaint();
                    thread::sleep(Duration::from_secs(1));
                }
            });

            Ok(Box::new(MainWindow::new(rx)))
        }),
    )?;

    Ok(())
}

fn set_styles(ctx: &Context) {
    let mut style = (*ctx.style()).clone();
    style.visuals = Visuals::dark();
    style.visuals.window_fill = egui::Color32::from_rgb(20, 20, 30);

    // Modify the font for buttons ---MAYBE DELETE LATER---
    // if let Some(text_style) = style.text_styles.get_mut(&egui::TextStyle::Button) {
    //     text_style.size = 14.0;
    //     text_style.family = egui::FontFamily::Monospace;
    // }
    //
    // // Modify the font for headings
    // if let Some(text_style) = style.text_styles.get_mut(&egui::TextStyle::Heading) {
    //     text_style.size = 34.0;
    // }

    ctx.set_style(style);
}
