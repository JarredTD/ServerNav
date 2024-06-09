use eframe::egui::{self, CentralPanel, Visuals};
use server_nav_ssh::connection::connect::connect_to_ssh;
use server_nav_ssh::connection::disconnect::{self, disconnect_ssh};
use ssh2::Session;

#[derive(Default)]
pub struct ServerNavApp {
    address: String,
    username: String,
    password: String,
    message: String,
    show_popup: bool,
    session: Option<Session>,
}

impl ServerNavApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for ServerNavApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Menu Bar
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Connect").clicked() {
                        self.show_popup = true
                    }
                    if ui.button("Disconnect").clicked() {
                        disconnect_ssh(self.session.take(), &mut self.message);
                    }
                    if ui.button("Quit").clicked() {
                        disconnect_ssh(self.session.take(), &mut self.message);
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
                });
                ui.add_space(16.0);
            });
        });
        // Connection Popup
        if self.show_popup {
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
                            ui.text_edit_singleline(&mut self.password);
                        });
                        if ui.button("Connect").clicked() {
                            disconnect_ssh(self.session.take(), &mut self.message);

                            let full_address = format!("{}:22", self.address);
                            self.session = connect_to_ssh(
                                full_address.as_str(),
                                &self.username,
                                &self.password,
                                &mut self.message,
                            );
                            self.show_popup = false;
                        }
                        if ui.button("Cancel").clicked() {
                            self.show_popup = false;
                        }
                    });
                });
        }
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(&self.message);
            });
        });
    }
}
