use serde::{Deserialize, Serialize};

use firecore_input::keyboard::{KeyMap, default_key_map};

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
			controls: default_key_map(),
			touchscreen: false,
		}
    }
}

impl PersistantData for Configuration {
    fn path() -> &'static str {
        "config.ron"
    }
}