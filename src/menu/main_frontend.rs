use eframe::{egui, epi};
use super::main_backend::MainBackend;
use super::main_frontend_collection::MainFrontendCollection;

// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct App {
    backend: MainBackend,
    collection: MainFrontendCollection,
}

// Impl here if needed
impl Default for App {
    fn default() -> Self {
        Self {
            backend: MainBackend {},
            collection: MainFrontendCollection::default(),
        }
    }
}

impl epi::App for App {
    fn name(&self) -> &str {
        "Adunis" // Name displayed in the OS bar of the window
    }

    /// Called once before the first frame
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
            self.collection.assert_theme(_ctx);
        }
    }

    /// Called by the frame work to save state before shutdown
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second
    fn update(&mut self, ctx: &egui::CtxRef, _frame: &epi::Frame) {

        let window_bounds = ctx.available_rect();
        let window_width = window_bounds.max.x - window_bounds.min.x;

        self.collection.assert_font(ctx, window_width);
        
        egui::SidePanel::left("side_panel").resizable(false).show(ctx, |ui| {

            egui::warn_if_debug_build(ui);
            ui.set_width(window_width / 3.4);

            ui.vertical_centered(|ui| {

                self.collection.format_menu_panel(ui);

                // Buttons within the menu
                if ui.button("Play").clicked() {  
                    self.backend.play();
                }
                if ui.button("Create").clicked() {  
                    self.backend.create();
                }
                if ui.button("Settings").clicked() {  
                    self.backend.settings();
                }
                if ui.button("Exit").clicked() {  
                    self.backend.exit(_frame);
                }
                if ui.button("Mode toggle").clicked() {  
                    self.collection.theme_shift(ctx);
                }
            });

        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.collection.generate_background_panel(ui);
        });

    }
}