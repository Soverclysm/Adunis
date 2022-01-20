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

        egui::CentralPanel::default().show(ctx, |ui| {

            egui::warn_if_debug_build(ui); // Display in the top left if the current build is a debug build

            // Create the central menu selection
            ui.vertical_centered(|ui| {
                let space = ui.available_height() / 2.5;
                ui.style_mut().spacing.item_spacing = egui::Vec2::new(space / 12.0, space / 12.0);
                ui.add_space(space);
                ui.heading("dndthing");
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

    }
}