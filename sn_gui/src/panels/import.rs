use crate::app::ServerNavApp;
use eframe::egui::{self, Context};

impl ServerNavApp {
    pub fn show_import_menu(&mut self, ctx: &Context) {
        egui::Window::new("Import File")
            .collapsible(false)
            .resizable(false)
            .default_pos(egui::pos2(
                ctx.available_rect().center().x - 150.0,
                ctx.available_rect().center().y - 100.0,
            ))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    if let Some(selected_path) = &self.selected_import_path {
                        ui.horizontal(|ui| {
                            ui.label("Selected file:");
                            ui.monospace(selected_path.display().to_string());
                        });
                    }
                    if ui.button("Open file…").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            self.selected_import_path = Some(path);
                        }
                    }
                });
            });
    }
}
