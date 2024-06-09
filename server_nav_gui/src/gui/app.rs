use crate::gui::panels::connection::show_connection_menu;
use crate::gui::panels::file_tree::show_file_tree;
use crate::gui::panels::menu::show_menu;
use crate::gui::panels::message::show_messages;

use eframe::egui::{self};
use ssh2::Session;
use std::path::PathBuf;

#[derive(Default)]
pub struct ServerNavApp {
    pub address: String,
    pub username: String,
    pub password: String,
    pub message: String,
    pub current_wd: PathBuf,
    pub show_popup: bool,
    pub show_hidden_files: bool,
    pub session: Option<Session>,
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
        show_menu(ctx, self);
        // Connection Popup
        show_connection_menu(ctx, self);
        // File Tree
        show_file_tree(ctx, self);
        // Messages
        show_messages(ctx, self)
    }
}
