#![cfg_attr(
    not(debug_assertions),
    windows_subsystem = "windows"
)] // hide console window on Windows in release

use eframe::egui::{self, CentralPanel, Context, TopBottomPanel, Visuals};

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([520.0, 340.0])
            .with_min_inner_size([300.0, 240.0]),
            // .with_decorations(false) // Disable window decorations
            // .with_transparent(true), // Enable transparency
        ..Default::default()
    };
    eframe::run_native(
        "HW monitor",
        options,
        Box::new(|cc| {
            // This gives us image support:
            set_styles(&cc.egui_ctx);
            Ok(Box::new(MainWindow::new()))
        }),
    )
}


fn set_styles(ctx: &Context) {
    let mut style = (*ctx.style()).clone();
    style.visuals = Visuals::dark();
    style.visuals.window_fill = egui::Color32::from_rgb(20, 20, 30);

    // Modify the font for buttons
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
struct MainWindow {
    name: String,
    age: u32,
    scale: f32,
}

impl MainWindow {
    fn new() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
            scale: 1.1,
        }
    }
}

impl eframe::App for MainWindow {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(self.scale);

        TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {

                    egui::widgets::global_theme_preference_switch(ui);
                    ui.label("Theme");
                });
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));

            if ui.button(egui::RichText::new("Increment").size(16.0)).clicked() {
                self.age += 1;
            }

            let label_text = format!("Hello '{}', age {}", self.name, self.age);
            ui.label(egui::RichText::new(label_text).size(20.0));
        });
    }
}
