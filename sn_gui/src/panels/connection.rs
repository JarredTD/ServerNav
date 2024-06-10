use crate::app::ServerNavApp;
use eframe::egui::{self, Context};
use sn_ssh::connection::{connect_to_ssh, disconnect_ssh};
use sn_ssh::file_ops::get_working_dir;

impl ServerNavApp {
    pub fn show_connection_menu(&mut self, ctx: &Context) {
        if self.show_popup || self.session.is_none() {
            egui::Window::new("New Connection")
                .collapsible(false)
                .resizable(false)
                .default_pos(egui::pos2(
                    ctx.available_rect().center().x - 150.0,
                    ctx.available_rect().center().y - 100.0,
                ))
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.label("Address:");
                            ui.text_edit_singleline(&mut self.address);
                        });
                        ui.horizontal(|ui| {
                            ui.label("Username:");
                            ui.text_edit_singleline(&mut self.username);
                        });
                        ui.horizontal(|ui| {
                            ui.label("Password:");
                            ui.add(egui::TextEdit::singleline(&mut self.password).password(true));
                        });
                        if ui.button("Connect").clicked() {
                            match disconnect_ssh(self.session.take()) {
                                Ok(msg) => self.message = msg,
                                Err(err) => self.message = err,
                            }

                            let full_address = format!("{}:22", self.address);
                            self.session = match connect_to_ssh(
                                full_address.as_str(),
                                &self.username,
                                &self.password,
                            ) {
                                Ok(Some(session)) => {
                                    self.message =
                                        format!("Successfully connected to {}", self.address);
                                    Some(session)
                                }
                                Ok(None) => {
                                    self.message =
                                        format!("No connection made to {}", self.address);
                                    None
                                }
                                Err(err) => {
                                    self.message = err.to_string();
                                    None
                                }
                            };
                            if let Some(session) = &self.session {
                                match get_working_dir(session) {
                                    Ok(wd) => {
                                        self.current_wd = Some(wd);
                                    }
                                    Err(err) => {
                                        self.message = err;
                                    }
                                };
                            }
                            self.show_popup = false;
                            self.reset_connection_lables()
                        }
                        if ui.button("Cancel").clicked() {
                            self.show_popup = false;
                            self.reset_connection_lables()
                        }
                    });
                });
        }
    }

    fn reset_connection_lables(&mut self) {
        self.address = String::new();
        self.username = String::new();
        self.password = String::new();
    }
}
