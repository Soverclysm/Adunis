use eframe::{egui, epi};
use super::main_interface::MainInterface;
use egui::Visuals;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
// Struct here if needed
pub struct App {
    interface: MainInterface,
    dark_mode: bool
}

// Impl here if needed
impl Default for App {
    fn default() -> Self {
        Self {
            interface: MainInterface {},
            dark_mode: true
        }
    }
}

impl epi::App for App {
    fn name(&self) -> &str {
        "Adunis" // Name displayed in the OS bar of the window
    }

    // Called once before the first frame
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // Load previous app state (if any).
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default();
    
            if self.dark_mode {
            _ctx.set_visuals(Visuals::dark());
            }   
            else {
                _ctx.set_visuals(Visuals::light());
            }
        }

    }

    // Called by the frame work to save state before shutdown
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second
    fn update(&mut self, ctx: &egui::CtxRef, _frame: &epi::Frame) {
        //let Self { label, value } = self; This line is not necessary until the struct is necessary

        let window_bounds = ctx.available_rect();
        let window_width = window_bounds.max.x - window_bounds.min.x;

        // Get font
        let mut fonts = egui::FontDefinitions::default();

        // Set font size to window_width / 36 for button text
        fonts.family_and_size.insert(
        egui::TextStyle::Button,
        (egui::FontFamily::Proportional, window_width / 36.0));

        // Set font size to window_width / 36 for title text
        fonts.family_and_size.insert(
            egui::TextStyle::Heading,
            (egui::FontFamily::Proportional, window_width / 30.0));

        // Apply font size and family changes
        ctx.set_fonts(fonts);
        
        egui::SidePanel::left("side_panel").resizable(false).show(ctx, |ui| {
            egui::warn_if_debug_build(ui);
            ui.set_width(window_width / 3.4);
            ui.vertical_centered(|ui| {
                let space = ui.available_height() / 4.0;
                ui.style_mut().spacing.item_spacing = egui::Vec2::new(space / 12.0, space / 12.0);
                ui.style_mut().spacing.button_padding = egui::Vec2::new(space / 12.0, space / 20.0);
                //ui.shrink_width_to_current(); trying to get the fucking buttons to all be the same width but no property for some reason fuck sake
                ui.add_space(space);
                ui.heading("Adunnis");
                ui.add_space(space / 5.0);
                if ui.button("Play").clicked() {  
                    self.interface.play();
                }
                if ui.button("Create").clicked() {  
                    self.interface.create();
                }
                if ui.button("Settings").clicked() {  
                    self.interface.settings();
                }
                if ui.button("Exit").clicked() {  
                    self.interface.exit(_frame);
                }
                if ui.button("Mode toggle").clicked() {  
                    self.dark_mode = !self.dark_mode;
                    let mut visuals_type = Visuals::light();
                    if self.dark_mode {
                        visuals_type = Visuals::dark();
                    }
                    ctx.set_visuals(visuals_type);
                }
            });

        });

        egui::CentralPanel::default().show(ctx, |ui| {

            let texture_id = egui::TextureId::Egui; // I don't know how to make this an actual iamge, I don't even know if it's possible

            ui.vertical_centered(|ui| {
                let space = ui.available_height() / 7.0;
                ui.add_space(space);
                ui.image(texture_id, [ui.available_width() / 2.0, ui.available_width() / 2.0]);
            });

        });

    }
}