
mod menufront_main; // Class for frontend of main menu
mod menuinterface_main;

fn main() {

    // Main Menu initialisation code
    let app = menufront_main::DndthingApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);

}