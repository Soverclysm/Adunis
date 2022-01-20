
mod menu;
use menu::main_frontend;

fn main() {

    // Main Menu initialisation code
    let app = main_frontend::App::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);

}