use egui::Visuals;

#[derive(Default)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
/// This struct is used to define parts of the main_frontend.rs file - 
/// ones that are particularly long or otherwise unclean to include in a file
/// oriented at UI code. The semantics of how this UI is achieved specifically
/// is not important to the frontend file.
pub struct MainFrontendCollection {
    dark_mode: bool
}

impl MainFrontendCollection {

    pub fn default() -> Self {
        Self {
            dark_mode: true
        }
    }

    /// This functions shifts the boolean of dark_mode and then asserts the new theme
    pub fn theme_shift(&mut self, _ctx: &egui::CtxRef) {
        self.dark_mode = !self.dark_mode;
        self.assert_theme(_ctx);
    }

    /// Asserts the current theme saved as dark_mode into the given frame
    pub fn assert_theme(&mut self, _ctx: &egui::CtxRef) {
        match self.dark_mode {
            false => {_ctx.set_visuals(Visuals::light())}
            true => {_ctx.set_visuals(Visuals::dark())}
        }
    }

    /// Asserts font size to the context dependent on width of the window, 
    /// maybe should be changed for height too, but that's a bit of effort maybe
    pub fn assert_font(&self, ctx: &egui::CtxRef, window_width: f32) {
        // Get font
        let mut fonts = egui::FontDefinitions::default();

        // Set font size to window_width / 36 for button text
        fonts.family_and_size.insert(egui::TextStyle::Button, (egui::FontFamily::Proportional, window_width / 36.0));
 
        // Set font size to window_width / 30 for title text
        fonts.family_and_size.insert(egui::TextStyle::Heading, (egui::FontFamily::Proportional, window_width / 30.0));
 
        ctx.set_fonts(fonts);
    }

    /// Formats the side panel of the main menu, namely of the spacing and a heading
    pub fn format_menu_panel(&self, ui: &mut egui::Ui) {
        let space = ui.available_height() / 4.0;
        ui.style_mut().spacing.item_spacing = egui::Vec2::new(space / 12.0, space / 12.0);
        ui.style_mut().spacing.button_padding = egui::Vec2::new(space / 12.0, space / 20.0);
        ui.add_space(space);
        ui.heading("Adunis");
        ui.add_space(space / 5.0);
    }

    /// Creates the UI elements to be shown in the background of the menu 
    /// (within main_frontend this is the CentralPanel)
    pub fn generate_background_panel(&self, ui: &mut egui::Ui) {
        let texture_id = egui::TextureId::Egui; 
        ui.vertical_centered(|ui| {
            let space = ui.available_height() / 7.0;
            ui.add_space(space);
            ui.image(texture_id, [ui.available_width() / 2.0, ui.available_width() / 2.0]);
        });
    }

    pub fn get_mode_type(&mut self) -> &str {
        match self.dark_mode {
            true => {"☀"}
            false => {"🌙"}
        }
    }

}