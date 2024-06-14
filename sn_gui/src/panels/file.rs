use crate::app::ServerNavApp;
use eframe::egui::{self, CentralPanel, Context, ScrollArea};
use egui_extras::syntax_highlighting::{highlight, CodeTheme};
use sn_ssh::file_ops::file::read_file;
use std::path::Path;

impl ServerNavApp {
    pub fn show_file(&mut self, ctx: &Context) {
        if let (Some(session), Some(current_file)) = (&self.session, &self.current_file.clone()) {
            if self.file_text_buffer.content.is_none() {
                match read_file(session, current_file) {
                    Ok(contents) => {
                        let file_name = match current_file.file_name() {
                            Some(name) => name.to_string_lossy().to_string(),
                            None => String::from("Unknown"),
                        };
                        self.message = format!("Reading {}", file_name);
                        self.file_text_buffer.content = Some(contents.clone());
                        self.temp_text_buffer = contents
                    }
                    Err(err) => {
                        self.message =
                            format!("Unable to read {}: {}", current_file.display(), err);
                        self.current_file = None;
                    }
                }
            }
            CentralPanel::default().show(ctx, |ui| {
                let theme = CodeTheme::from_memory(ui.ctx());
                let mut layouter = |ui: &egui::Ui, text: &str, _: f32| {
                    let layout_job = highlight(ctx, &theme, text, &get_language(current_file));
                    ui.fonts(|fonts| fonts.layout_job(layout_job))
                };
                ScrollArea::vertical().show(ui, |ui| {
                    let editor = egui::TextEdit::multiline(&mut self.temp_text_buffer)
                        .font(egui::TextStyle::Monospace)
                        .code_editor()
                        .layouter(&mut layouter);

                    let response = ui.add(editor);

                    if response.changed() {
                        self.file_text_buffer
                            .update_content(self.temp_text_buffer.clone());
                    }
                });
            });
        }
    }
}

fn get_language(file: &Path) -> String {
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
