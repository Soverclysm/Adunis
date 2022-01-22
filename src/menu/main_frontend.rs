use eframe::{egui, epi};
use super::main_interface::MainInterface;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
// Struct here if needed
pub struct App {
    interface: MainInterface
}

// Impl here if needed
impl Default for App {
    fn default() -> Self {
        Self {
            interface: MainInterface {}
        }
    }
}

impl epi::App for App {
    fn name(&self) -> &str {
        "dndthing" // Name displayed in the OS bar of the window
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
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }

        // Get font
        let mut fonts = egui::FontDefinitions::default();

        // Set button font size to 20pt
        fonts.family_and_size.insert(
        egui::TextStyle::Button,
        (egui::FontFamily::Proportional, 20.0));

        // Set heading font size to 28pt
        fonts.family_and_size.insert(
            egui::TextStyle::Heading,
            (egui::FontFamily::Proportional, 28.0));

        // Apply font size and family changes
        _ctx.set_fonts(fonts);
    }

    // Called by the frame work to save state before shutdown
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second
    fn update(&mut self, ctx: &egui::CtxRef, _frame: &epi::Frame) {
        //let Self { label, value } = self; This line is not necessary until the struct is necessary

        
        egui::SidePanel::left("side_panel").resizable(false).show(ctx, |ui| {
            egui::warn_if_debug_build(ui);
            let _width_space = ui.available_width() / 2.0;
            ui.set_width(300.0);
            ui.vertical_centered(|ui| {
                let space = ui.available_height() / 4.0;
                ui.style_mut().spacing.item_spacing = egui::Vec2::new(space / 12.0, space / 12.0);
                ui.style_mut().spacing.button_padding = egui::Vec2::new(space / 12.0, space / 20.0);
                //ui.shrink_width_to_current(); trying to get the fucking buttons to all be the same width but no property for some reason fuck sake
                ui.add_space(space);
                ui.heading("dndthing");
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
                    self.interface.exit();
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