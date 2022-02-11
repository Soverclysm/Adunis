
#[derive(Default)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct MainInterface {

}

impl MainInterface {

    pub fn play(&self) {

    }

    pub fn create(&self) {

    }

    pub fn settings(&self) {

    }

    pub fn exit(&self, _frame: &epi::Frame) {
        _frame.quit();
    }

}