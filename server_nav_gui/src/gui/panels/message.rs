use crate::gui::app::ServerNavApp;
use eframe::egui::{self, Context};

pub fn show_messages(ctx: &Context, app: &mut ServerNavApp) {
    egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.label(&app.message);
        });
    });
}
