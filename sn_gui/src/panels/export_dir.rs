use crate::app::ServerNavApp;
use eframe::egui::{self, Align, Context, RichText};
use sn_ssh::file_ops::export_directory;

impl ServerNavApp {
    pub fn show_export_dir_menu(&mut self, ctx: &Context) {
        egui::Window::new("Export Directory")
            .collapsible(false)
            .resizable(false)
            .default_pos(egui::pos2(
                ctx.available_rect().center().x - 150.0,
                ctx.available_rect().center().y - 100.0,
            ))
            .title_bar(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Export Directory").strong());
                    ui.with_layout(egui::Layout::right_to_left(Align::Max), |ui| {
                        if ui.button("❌").clicked() {
                            self.show_export_dir_popup = false;
                        }
                    });
                });

                ui.separator();
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        if let Some(selected_dir) = &self.current_wd {
                            ui.label("Selected directory:");
                            ui.monospace(selected_dir.display().to_string());
                        }
                    });

                    if ui.button("Export").clicked() {
                        if let (Some(selected_dir), Some(session)) =
                            (&self.current_wd, &self.session)
                        {
                            match export_directory(session, selected_dir) {
                                Ok(()) => {
                                    self.message =
                                        format!("Successfully exported {}", selected_dir.display());
                                    self.show_export_dir_popup = false;
                                }
                                Err(err) => self.message = err.to_string(),
                            }
                        }
                    }
                });
            });
    }
}
