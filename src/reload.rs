use firecore_saves::PersistantData;
use crate::configuration::Configuration;
use macroquad::prelude::info;

pub trait Reloadable: PersistantData {

    fn on_reload(&self);

}

impl Reloadable for Configuration {
    fn on_reload(&self) {
        info!("Running configuration reload tasks...");
        firecore_input::keyboard::load(self.controls.clone());
        firecore_input::touchscreen::touchscreen(self.touchscreen);
        info!("Finished configuration reload tasks!");
    }
}