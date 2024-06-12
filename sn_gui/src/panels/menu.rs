use crate::app::ServerNavApp;
use eframe::egui::{self, Context, Visuals};
use sn_ssh::connection::disconnect_ssh;
use sn_ssh::file_ops::modify_file;

impl ServerNavApp {
    pub fn show_menu(&mut self, ctx: &Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Connect").clicked() {
                        self.show_connection_popup = true
                    }
                    if self.session.is_some() {
                        if ui.button("Disconnect").clicked() {
                            self.current_wd = None;
                            self.current_file = None;
                            match disconnect_ssh(self.session.take()) {
                                Ok(msg) => self.message = msg,
                                Err(err) => self.message = err,
                            }
                        }
                        if ui.button("Import File").clicked() {
                            self.show_import_popup = true;
                        }
                        if ui.button("Export File").clicked() {
                            self.show_import_popup = true;
                        }
                    }
                    if self.current_file.is_some() && ui.button("Save").clicked() {
                        if let (Some(content), Some(session), Some(current_file)) = (
                            &self.file_text_buffer.content,
                            &self.session,
                            &self.current_file,
                        ) {
                            match modify_file(session, current_file, content) {
                                Ok(()) => self.file_text_buffer.is_saved = true,
                                Err(err) => self.message = err,
                            }
                        }
                    }
                    if self.session.is_some() && ui.button("Quit").clicked() {
                        match disconnect_ssh(self.session.take()) {
                            Ok(_) => self.message = "".to_string(),
                            Err(err) => self.message = err,
                        }
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                ui.add_space(16.0);
                ui.menu_button("View", |ui| {
                    ui.menu_button("Theme", |ui| {
                        if ui.button("Dark").clicked() {
                            ctx.set_visuals(Visuals::dark());
                        }
                        if ui.button("Light").clicked() {
                            ctx.set_visuals(Visuals::light());
                        }
                    });
                    ui.menu_button("File Tree", |ui| {
                        ui.toggle_value(&mut self.show_hidden_files, "Show Hidden Files")
                    });
                });
                ui.add_space(16.0);
            });
        });
    }
}
