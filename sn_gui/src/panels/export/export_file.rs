use crate::app::ServerNavApp;
use eframe::egui::{self, Align, Context, RichText};
use sn_ssh::file_ops::export::export_file;

impl ServerNavApp {
    pub fn show_export_menu(&mut self, ctx: &Context) {
        egui::Window::new("Export File")
            .collapsible(false)
            .resizable(false)
            .default_pos(egui::pos2(
                ctx.available_rect().center().x - 150.0,
                ctx.available_rect().center().y - 100.0,
            ))
            .title_bar(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Export File").strong());
                    ui.with_layout(egui::Layout::right_to_left(Align::Max), |ui| {
                        if ui.button("❌").clicked() {
                            self.show_export_popup = false;
                        }
                    });
                });

                ui.separator();
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        if let Some(selected_path) = &self.current_file {
                            ui.horizontal(|ui| {
                                ui.label("Selected file:");
                                ui.monospace(selected_path.display().to_string());
                            });
                        }
                        if ui.button("Export").clicked() {
                            if let (Some(selected_file), Some(session)) =
                                (&self.current_file, &self.session)
                            {
                                if let Some(downloads_folder) = dirs::download_dir() {
                                    match export_file(session, selected_file, &downloads_folder) {
                                        Ok(()) => {
                                            self.message = format!(
                                                "Successfully exported {}",
                                                selected_file.display()
                                            );
                                            self.show_export_popup = false
                                        }
                                        Err(err) => self.message = err.to_string(),
                                    }
                                }
                            }
                        }
                    });
                });
            });
    }
}
