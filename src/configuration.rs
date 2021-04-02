use serde::{Deserialize, Serialize};

use firecore_input::KeyMap;

use firecore_saves::PersistantData;

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {

	#[serde(default)]
	pub controls: KeyMap,	

	#[serde(default)]
	pub touchscreen: bool,

}

impl Default for Configuration {
    fn default() -> Self {
        Self {
			controls: firecore_input::keyboard::default(),
			touchscreen: false,
		}
    }
}

impl PersistantData for Configuration {
    fn path() -> &'static str {
        "config.ron"
    }
}