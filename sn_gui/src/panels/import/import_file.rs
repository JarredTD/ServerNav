use crate::app::ServerNavApp;
use eframe::egui::{self, Align, Context, RichText};
use sn_ssh::file_ops::import::import_file;

impl ServerNavApp {
    pub fn show_import_menu(&mut self, ctx: &Context) {
        egui::Window::new("Import File")
            .collapsible(false)
            .resizable(false)
            .default_pos(egui::pos2(
                ctx.available_rect().center().x - 150.0,
                ctx.available_rect().center().y - 100.0,
            ))
            .title_bar(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Import File").strong());
                    ui.with_layout(egui::Layout::right_to_left(Align::Max), |ui| {
                        if ui.button("❌").clicked() {
                            self.show_import_popup = false;
                        }
                    });
                });

                ui.separator();
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        if let Some(selected_path) = &self.selected_import_path {
                            ui.horizontal(|ui| {
                                ui.label("Selected file:");
                                ui.monospace(selected_path.display().to_string());
                            });
                        }
                        ui.with_layout(egui::Layout::right_to_left(Align::Max), |ui| {
                            if ui.button("…").clicked() {
                                if let Some(path) = rfd::FileDialog::new().pick_file() {
                                    self.selected_import_path = Some(path);
                                }
                            }
                        });
                    });
                    if self.selected_import_path.is_some() && ui.button("Import").clicked() {
                        if let (Some(current_wd), Some(import_path), Some(session)) = (
                            &self.current_wd,
                            self.selected_import_path.take(),
                            &self.session,
                        ) {
                            match import_file(session, current_wd, &import_path) {
                                Ok(()) => {
                                    self.message =
                                        format!("Successfully imported {}", import_path.display());
                                    self.show_import_popup = false
                                }
                                Err(err) => self.message = err.to_string(),
                            }
                        }
                    }
                });
            });
    }
}
