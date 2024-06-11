use crate::app::ServerNavApp;
use eframe::egui::{self, Context};
use sn_ssh::file_ops::modify_file;

impl ServerNavApp {
    pub fn show_file_options(&mut self, ctx: &Context) {
        if let (Some(session), Some(current_file)) = (&self.session, &self.current_file.clone()) {
            egui::SidePanel::right("File Options").show(ctx, |ui| {
                if ui.button("Save").clicked() {
                    if let Some(content) = &self.file_text_buffer.content {
                        match modify_file(session, current_file, content) {
                            Ok(()) => self.file_text_buffer.is_saved = true,
                            Err(err) => self.message = err,
                        }
                    }
                }
            });
        }
    }
}
