use std::process;

#[derive(Default)]
pub struct MainInterface {

}

impl MainInterface {

    pub fn play(&self) {

    }

    pub fn create(&self) {

    }

    pub fn settings(&self) {

    }

    pub fn exit(&self) {
        process::exit(0);
    }

}