use crate::gui::app::ServerNavApp;
use eframe::egui::{self, Context, Visuals};
use server_nav_ssh::connection::disconnect::disconnect_ssh;
use std::path::PathBuf;

pub fn show_menu(ctx: &Context, app: &mut ServerNavApp) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Connect").clicked() {
                    app.show_popup = true
                }
                if ui.button("Disconnect").clicked() {
                    app.current_wd = PathBuf::new();
                    match disconnect_ssh(app.session.take()) {
                        Ok(msg) => app.message = msg,
                        Err(err) => app.message = err,
                    }
                }
                if ui.button("Quit").clicked() {
                    match disconnect_ssh(app.session.take()) {
                        Ok(_) => app.message = "".to_string(),
                        Err(err) => app.message = err,
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
                    ui.toggle_value(&mut app.show_hidden_files, "Show Hidden Files")
                });
            });
            ui.add_space(16.0);
        });
    });
}
