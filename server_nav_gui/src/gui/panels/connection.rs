use crate::gui::app::ServerNavApp;
use eframe::egui::{self, Context};
use server_nav_ssh::connection::connect::connect_to_ssh;
use server_nav_ssh::connection::disconnect::disconnect_ssh;
use server_nav_ssh::view::list::get_working_dir;

pub fn show_connection_menu(ctx: &Context, app: &mut ServerNavApp) {
    if app.show_popup {
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
                        ui.text_edit_singleline(&mut app.address);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Username:");
                        ui.text_edit_singleline(&mut app.username);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Password:");
                        ui.add(egui::TextEdit::singleline(&mut app.password).password(true));
                    });
                    if ui.button("Connect").clicked() {
                        match disconnect_ssh(app.session.take()) {
                            Ok(msg) => app.message = msg,
                            Err(err) => app.message = err,
                        }

                        let full_address = format!("{}:22", app.address);
                        app.session = match connect_to_ssh(
                            full_address.as_str(),
                            &app.username,
                            &app.password,
                        ) {
                            Ok(Some(session)) => {
                                app.message = format!("Successfully connected to {}", app.address);
                                Some(session)
                            }
                            Ok(None) => {
                                app.message = format!("No connection made to {}", app.address);
                                None
                            }
                            Err(err) => {
                                app.message = err.to_string();
                                None
                            }
                        };
                        if let Some(session) = &app.session {
                            match get_working_dir(session) {
                                Ok(wd) => {
                                    app.current_wd = Some(wd);
                                }
                                Err(err) => {
                                    app.message = err;
                                }
                            };
                        }
                        app.show_popup = false;
                    }
                    if ui.button("Cancel").clicked() {
                        app.show_popup = false;
                    }
                });
            });
    }
}
