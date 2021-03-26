use macroquad::prelude::{info, warn};
use std::path::PathBuf;
use crate::data::{PersistantData, PersistantDataLocation};

use async_trait::async_trait;

use firecore_data_lib::configuration::Configuration;

const CONFIGURATION_PATH: &str = "config";
const CONFIGURATION_FILENAME: &str = "config.ron";

pub fn on_reload(config: &Configuration) {
	info!("Running configuration reload tasks...");
	*firecore_input::keyboard::KEY_CONTROLS.write() = firecore_input::keyboard::serialization::normal_map(&config.controls);
	info!("Finished configuration reload tasks!");
}

fn saved_default() -> Configuration {
	macroquad::prelude::info!("Creating new configuration file.");
	let default = config_default();
	default.save();
	return default;
}

fn default_path() -> PathBuf {
	PathBuf::from(CONFIGURATION_PATH).join(CONFIGURATION_FILENAME)
}

fn config_default() -> Configuration {
	Configuration {
		controls: firecore_input::keyboard::serialization::ser_map(firecore_input::keyboard::default()),
		// touchscreen: false,
	}
}

#[async_trait(?Send)]
impl PersistantDataLocation for Configuration {

	async fn load_from_file() -> Self {
		Configuration::load(default_path()).await
	}

}

#[async_trait(?Send)]
impl PersistantData for Configuration {

	async fn load(path: PathBuf) -> Self {
		return match crate::data::read_string(path).await {
			Ok(content) => ron::from_str(&content).unwrap_or(saved_default()),
			Err(err) => {
				warn!("Failed reading configuration file to string with error {}", err);
				saved_default()
			}
		};
	}

	fn save(&self) {
		crate::data::save_struct(default_path(), &self);
	}

	async fn reload(&mut self) {
		info!("Attempting to reload configuration...");
		*self = Configuration::load_from_file().await;
		info!("Reloaded configuration!");
		on_reload(self);
	}
	
}