use std::path::{Path, PathBuf};

use eframe::egui::{self, Label, Sense, Visuals};
use server_nav_ssh::connection::connect::connect_to_ssh;
use server_nav_ssh::connection::disconnect::disconnect_ssh;
use server_nav_ssh::view::list::{get_working_dir, list_dir};
use ssh2::Session;

#[derive(Default)]
pub struct ServerNavApp {
    address: String,
    username: String,
    password: String,
    message: String,
    current_wd: PathBuf,
    show_popup: bool,
    show_hidden_files: bool,
    session: Option<Session>,
}

impl ServerNavApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for ServerNavApp {
    //TODO Refactor into smaller functions
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Menu Bar
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Connect").clicked() {
                        self.show_popup = true
                    }
                    if ui.button("Disconnect").clicked() {
                        self.current_wd = PathBuf::new();
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
                                        self.current_wd = wd.clone();
                                    }
                                    Err(err) => {
                                        self.message = err;
                                    }
                                };
                            }
                            self.show_popup = false;
                        }
                        if ui.button("Cancel").clicked() {
                            self.show_popup = false;
                        }
                    });
                });
        }
        if let Some(session) = &self.session {
            egui::SidePanel::left("File Tree").show(ctx, |ui| {
                ui.label(
                    egui::RichText::new(&self.current_wd.to_string_lossy().to_string())
                        .text_style(egui::TextStyle::Small)
                        .strong(),
                );
                if ui.button("<<").clicked() {
                    let current_path = Path::new(&self.current_wd);
                    if let Some(parent) = current_path.parent() {
                        self.current_wd = match parent.canonicalize() {
                            Ok(parent_path) => parent_path,
                            Err(err) => {
                                self.message = format!("Failed to resolve path: {}", err);
                                self.current_wd.clone()
                            }
                        };
                        self.message = format!(
                            "Moved up to: {}",
                            self.current_wd.to_string_lossy().to_string()
                        );
                    } else {
                        self.message = "Already at the root directory".to_string();
                    }
                }
                egui::ScrollArea::vertical().show(ui, |ui| {
                    let paths = match list_dir(session, &self.current_wd) {
                        Ok(paths) => paths,
                        Err(err) => {
                            self.message = err;
                            Vec::new()
                        }
                    };

                    let current_dir_clone = self.current_wd.clone();
                    for (path, stat) in &paths {
                        let file_name = match path.file_name() {
                            Some(name) => name.to_string_lossy().to_string(),
                            None => continue,
                        };

                        if !self.show_hidden_files {
                            if file_name.starts_with(".") {
                                continue;
                            }
                        }
                        if path == Path::new(&current_dir_clone) {
                            continue;
                        }

                        if ui
                            .add(Label::new(file_name).sense(Sense::click()))
                            .clicked()
                        {
                            if stat.is_dir() {
                                match path.canonicalize() {
                                    Ok(full_path) => {
                                        self.current_wd = full_path;
                                    }
                                    Err(err) => {
                                        self.message = format!("Failed to resolve path: {}", err)
                                    }
                                }
                            } else {
                                self.message =
                                    format!("{} is a file", path.to_string_lossy().to_string())
                            }
                        }
                    }
                });
            });
        }

        // BottomPanel
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(&self.message);
            });
        });
    }
}
