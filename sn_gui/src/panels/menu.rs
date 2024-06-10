use crate::app::ServerNavApp;
use eframe::egui::{self, Context, Visuals};
use sn_ssh::connection::disconnect_ssh;

impl ServerNavApp {
    pub fn show_menu(&mut self, ctx: &Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Connect").clicked() {
                        self.show_popup = true
                    }
                    if ui.button("Disconnect").clicked() {
                        self.current_wd = None;
                        self.current_file = None;
                        match disconnect_ssh(self.session.take()) {
                            Ok(msg) => self.message = msg,
                            Err(err) => self.message = err,
                        }
                    }
                    if ui.button("Quit").clicked() {
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
