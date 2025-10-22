use std::sync::mpsc::Receiver;
use std::time::Duration;
use crate::collectors::cpu_collector::CpuData;
use eframe::egui::{self, CentralPanel, Context, RichText, TopBottomPanel};
use sysinfo::System;
pub struct MainWindow {
    rx: Receiver<CpuData>,
    cpu_data: CpuData,
    scale: f32,
}

impl MainWindow {
    pub fn new(rx: Receiver<CpuData>) -> Self {
        Self {
            rx,
            cpu_data: CpuData::default(),
            scale: 1.1,
        }
    }
}

impl eframe::App for MainWindow {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        if let Ok(new_data) = self.rx.try_recv() {
            self.cpu_data = new_data;
        }

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
                        if theme == egui::ThemePreference::System {
                            if ui.ctx().style().visuals.dark_mode {
                                theme = egui::ThemePreference::Dark;
                            } else {
                                theme = egui::ThemePreference::Light;
                            }
                        }
                        let original_theme = theme;

                        ui.selectable_value(&mut theme, egui::ThemePreference::Light, "☀ Light");
                        ui.selectable_value(&mut theme, egui::ThemePreference::Dark, "🌙 Dark");

                        if theme != original_theme {
                            ui.ctx().options_mut(|o| o.theme_preference = theme);
                        }
                    });
                });
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            ui.heading(RichText::new(self.cpu_data.get_name()).strong().size(15.0));
            ui.separator();
            ui.label(RichText::new(format!(
                "Logical Cores: {}  @{:.2}hz",
                self.cpu_data.get_count(),
                self.cpu_data.get_base_frequency()
            )));
            ui.label(RichText::new(format!(
                "CPU Usage: {:.2}%",
                self.cpu_data.get_cpu_usage()
            )));
        });
    }
}
