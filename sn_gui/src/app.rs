use eframe::egui::{self};
use sn_ssh::buffer::text_buffer::TextBuffer;
use ssh2::Session;
use std::path::PathBuf;

#[derive(Default)]
pub struct ServerNavApp {
    pub address: String,
    pub username: String,
    pub password: String,
    pub message: String,
    pub current_wd: Option<PathBuf>,
    pub current_file: Option<PathBuf>,
    pub file_text_buffer: TextBuffer,
    pub temp_text_buffer: String,
    pub show_connection_popup: bool,
    pub show_import_popup: bool,
    pub show_export_popup: bool,
    pub show_hidden_files: bool,
    pub session: Option<Session>,
    pub selected_import_path: Option<PathBuf>,
}

impl ServerNavApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    fn show_ui(&mut self, ctx: &egui::Context) {
        // Menu Bar
        self.show_menu(ctx);
        // Connection Popup
        if self.show_connection_popup || self.session.is_none() {
            self.show_connection_menu(ctx);
        }
        // Import Popup
        if self.show_import_popup {
            self.show_import_menu(ctx)
        }
        // File Tree
        self.show_file_tree(ctx);
        // File
        self.show_file(ctx);
        // Messages
        self.show_messages(ctx);
    }
}

impl eframe::App for ServerNavApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.show_ui(ctx);
    }
}
