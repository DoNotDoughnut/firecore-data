extern crate firecore_input as input;
extern crate firecore_data as data;

use serde::{Deserialize, Serialize};

use input::keyboard::{KeyMap, default_key_map};

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {

	#[serde(default = "default_key_map")]
	pub controls: KeyMap,	

	#[serde(default)]
	pub touchscreen: bool,

}

impl Default for Configuration {
    fn default() -> Self {
        Self {
			controls: default_key_map(),
			touchscreen: false,
		}
    }
}

impl data::PersistantData for Configuration {
    fn file_name() -> &'static str {
        "config"
    }
}

impl data::Reloadable for Configuration {
    fn on_reload(&self) {
        use data::macroquad::prelude::info;
        info!("Running configuration reload tasks...");
        firecore_input::keyboard::load(self.controls.clone());
        firecore_input::touchscreen::touchscreen(self.touchscreen);
        info!("Finished configuration reload tasks!");
    }
}