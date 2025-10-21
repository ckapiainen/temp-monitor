#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod app;
mod collectors;

use app::ui_main_window::MainWindow;
use eframe::egui::{self, Context, Visuals};

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
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
            set_styles(&cc.egui_ctx);
            Ok(Box::new(MainWindow::new()))
        }),
    )
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
