use crate::app::ServerNavApp;
use eframe::egui::{self, Context, Label, Sense};
use sn_ssh::file_ops::directory::list_dir;
use std::path::Path;

impl ServerNavApp {
    pub fn show_file_tree(&mut self, ctx: &Context) {
        if let Some(session) = &self.session {
            egui::SidePanel::left("File Tree").show(ctx, |ui| {
                if let Some(current_wd) = &self.current_wd.clone() {
                    ui.label(
                        egui::RichText::new(current_wd.to_string_lossy().to_string())
                            .text_style(egui::TextStyle::Small)
                            .strong(),
                    );
                    if ui.button("<<").clicked() {
                        let current_path = Path::new(current_wd);
                        if let Some(parent) = current_path.parent() {
                            self.current_wd = match parent.canonicalize() {
                                Ok(parent_path) => Some(parent_path),
                                Err(err) => {
                                    self.message = format!("Failed to resolve path: {}", err);
                                    self.current_wd.clone()
                                }
                            };
                            self.message = format!(
                                "Moved up to: {}",
                                self.current_wd.as_ref().unwrap().display()
                            );
                        } else {
                            self.message = "Already at the root directory".to_string();
                        }
                    }

                    egui::ScrollArea::vertical().show(ui, |ui| {
                        let paths = match list_dir(session, current_wd) {
                            Ok(paths) => paths,
                            Err(err) => {
                                self.message = err;
                                Vec::new()
                            }
                        };

                        let current_dir_clone = current_wd.clone();
                        for (path, stat) in &paths {
                            let file_name = match path.file_name() {
                                Some(name) => name.to_string_lossy().to_string(),
                                None => continue,
                            };

                            if !self.show_hidden_files && file_name.starts_with('.') {
                                continue;
                            }
                            if path == Path::new(&current_dir_clone) {
                                continue;
                            }

                            if ui
                                .add(Label::new(file_name).sense(Sense::click()))
                                .clicked()
                            {
                                let full_path = match path.canonicalize() {
                                    Ok(full_path) => full_path,
                                    Err(err) => {
                                        self.message = format!("Failed to resolve path: {}", err);
                                        continue;
                                    }
                                };
                                if stat.is_dir() {
                                    self.current_wd = Some(full_path)
                                } else if stat.is_file() {
                                    self.current_file = Some(full_path);
                                    self.file_text_buffer.content = None;
                                }
                            }
                        }
                    });
                }
            });
        }
    }
}
