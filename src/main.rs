
mod app;

fn main() {
    println!("Hello world!");

    let app = app::DndthingApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);

}