use eframe;
use server_nav_gui::gui::app::ServerNavApp;

fn main() {
    let native_options = eframe::NativeOptions::default();
    _ = eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Box::new(ServerNavApp::new(cc))),
    );
}
