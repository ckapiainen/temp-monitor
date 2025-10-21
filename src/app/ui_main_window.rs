use eframe::egui::{self, CentralPanel, Context, TopBottomPanel};
use sysinfo::System;
use crate::collectors::cpu_collector::CpuData;

pub struct MainWindow {
    system: System,
    cpu_data: CpuData,
    scale: f32,
}

impl MainWindow {
    pub fn new() -> Self {
        let mut system = System::new_all();
        let cpu_data = CpuData::new(&system);

        Self {
            system,
            cpu_data,
            scale: 1.1,
        }
    }
}

impl eframe::App for MainWindow {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Refresh system data if needed
        self.system.refresh_cpu_all();
        self.cpu_data.update(&self.system);

        ctx.set_pixels_per_point(self.scale);
        TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            // Menu button
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("Menu", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                // Theme selector
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.horizontal(|ui| {
                        let mut theme = ui.ctx().options(|o| o.theme_preference);

                        if ui.selectable_label(theme == egui::ThemePreference::Light, "☀ Light").clicked() {
                            ui.ctx().options_mut(|o| o.theme_preference = egui::ThemePreference::Light);
                        }
                        if ui.selectable_label(theme == egui::ThemePreference::Dark, "🌙 Dark").clicked() {
                            ui.ctx().options_mut(|o| o.theme_preference = egui::ThemePreference::Dark);
                        }
                    });
                });
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            ui.heading(egui::RichText::new(self.cpu_data.get_name()).strong().size(20.0));
            ui.label(format!("Logical Cores: {}", self.cpu_data.get_count()));

        });
    }
}