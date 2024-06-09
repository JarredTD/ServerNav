use crate::gui::app::ServerNavApp;
use eframe::egui::{self, Context, Label, Sense};
use server_nav_ssh::view::list::list_dir;
use std::path::Path;

pub fn show_file_tree(ctx: &Context, app: &mut ServerNavApp) {
    if let Some(session) = &app.session {
        egui::SidePanel::left("File Tree").show(ctx, |ui| {
            if let Some(current_wd) = &app.current_wd.clone() {
                ui.label(
                    egui::RichText::new(&current_wd.to_string_lossy().to_string())
                        .text_style(egui::TextStyle::Small)
                        .strong(),
                );
                if ui.button("<<").clicked() {
                    let current_path = Path::new(current_wd);
                    if let Some(parent) = current_path.parent() {
                        app.current_wd = match parent.canonicalize() {
                            Ok(parent_path) => Some(parent_path),
                            Err(err) => {
                                app.message = format!("Failed to resolve path: {}", err);
                                app.current_wd.clone()
                            }
                        };
                        app.message = format!(
                            "Moved up to: {}",
                            app.current_wd
                                .as_ref()
                                .unwrap()
                                .to_string_lossy()
                                .to_string()
                        );
                    } else {
                        app.message = "Already at the root directory".to_string();
                    }
                }

                egui::ScrollArea::vertical().show(ui, |ui| {
                    let paths = match list_dir(session, current_wd) {
                        Ok(paths) => paths,
                        Err(err) => {
                            app.message = err;
                            Vec::new()
                        }
                    };

                    let current_dir_clone = current_wd.clone();
                    for (path, stat) in &paths {
                        let file_name = match path.file_name() {
                            Some(name) => name.to_string_lossy().to_string(),
                            None => continue,
                        };

                        if !app.show_hidden_files && file_name.starts_with(".") {
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
                                    app.message = format!("Failed to resolve path: {}", err);
                                    continue;
                                }
                            };
                            if stat.is_dir() {
                                app.current_wd = Some(full_path)
                            } else if stat.is_file() {
                                app.current_file = Some(full_path)
                            }
                        }
                    }
                });
            }
        });
    }
}
