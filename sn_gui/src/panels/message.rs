use crate::app::ServerNavApp;
use eframe::egui::{self, Context};

impl ServerNavApp {
    pub fn show_messages(&mut self, ctx: &Context) {
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(&self.message);
            });
        });
    }
}
