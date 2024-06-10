#![windows_subsystem = "windows"]

use eframe;
use sn_gui::app::ServerNavApp;

fn main() {
    let native_options = eframe::NativeOptions::default();
    _ = eframe::run_native(
        "ServerNav",
        native_options,
        Box::new(|cc| Box::new(ServerNavApp::new(cc))),
    );
}
