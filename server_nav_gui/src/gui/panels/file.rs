use std::path::PathBuf;

use crate::gui::app::ServerNavApp;
use eframe::egui::{self, CentralPanel, Context, ScrollArea};
use egui_extras::syntax_highlighting::{highlight, CodeTheme};
use server_nav_ssh::view::list::read_file;

pub fn show_file(ctx: &Context, app: &mut ServerNavApp) {
    if let (Some(session), Some(current_file)) = (&app.session, &app.current_file) {
        match read_file(session, current_file) {
            Ok(contents) => {
                let file_name = match current_file.file_name() {
                    Some(name) => name.to_string_lossy().to_string(),
                    None => String::from("Unknown"),
                };
                app.message = format!("Reading {}", file_name);
                CentralPanel::default().show(ctx, |ui| {
                    let theme = CodeTheme::from_memory(ui.ctx());
                    ScrollArea::vertical().show(ui, |ui| {
                        ui.add(
                            egui::TextEdit::multiline(&mut contents.to_string())
                                .font(egui::TextStyle::Monospace)
                                .code_editor()
                                .layouter(&mut |ui, text, _| {
                                    let layout_job =
                                        highlight(ctx, &theme, text, &get_language(current_file));
                                    ui.fonts(|fonts| fonts.layout_job(layout_job))
                                }),
                        );
                    });
                });
            }
            Err(err) => {
                app.message = format!(
                    "Unable to read {}: {}",
                    current_file.to_string_lossy().to_string(),
                    err
                );
                app.current_file = None;
            }
        }
    }
}

fn get_language(file: &PathBuf) -> String {
    match file.extension().and_then(|ext| ext.to_str()) {
        Some("rs") => "rust",
        Some("c") | Some("h") => "c",
        Some("cpp") | Some("hpp") => "cpp",
        Some("py") => "python",
        Some("toml") => "toml",
        Some("txt") | None => "text",
        _ => "text",
    }
    .to_string()
}