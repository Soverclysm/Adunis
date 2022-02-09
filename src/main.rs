
mod menu;
use menu::main_frontend;
use egui::Visuals;

fn main() {

    // Main Menu initialisation code
    let app = main_frontend::App::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);

}


//#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AppState {
    pub search_filter: String,
    //#[serde(skip)]
    pub do_scroll_to_selected_log: bool,
    //#[serde(skip)]
    pub is_about_open: bool,
    //#[serde(skip)]
    pub is_settings_open: bool,
    pub is_autosave: bool,
    pub is_case_sensitive: bool,
    pub is_copying_line_indicator: bool,
    pub is_copying_line_numbers: bool,
    pub is_message_preview_open: bool,
    pub is_newest_first: bool,
    pub is_using_regex: bool,
    //#[serde(skip)]
    //#[serde(skip)]
    pub preview_height: f32,
    //#[serde(skip)]
    pub current_theme: Visuals,
    //#[serde(skip)]
    pub copy_language: String,
    //#[serde(skip)]
    pub alert_string: String,
}