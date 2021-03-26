use std::path::PathBuf;

use firecore_data_lib::player::list::PlayerSaves;
use macroquad::prelude::{info, warn};
use crate::data::{PersistantData, PersistantDataLocation};

const FILE: &str = "saves.ron";

#[async_trait::async_trait(?Send)]
impl PersistantDataLocation for PlayerSaves {
    async fn load_from_file() -> Self {
        Self::load(PathBuf::from(FILE)).await
    }
}

#[async_trait::async_trait(?Send)]
impl PersistantData for PlayerSaves {

    async fn load(path: PathBuf) -> Self {
        info!("Loading player data...");
		match crate::data::read_string(&path).await {
			Ok(data) => {
				match ron::from_str(&data) {
				    Ok(data) => data,
				    Err(err) => {
						warn!("Could not read player data with error {}", err);
                        Self::default()
					}
				}
			},
		    Err(err) => {
				warn!("Could not open player data file at {:?} with error {}", path, err);
                Self::default()
			}
		}
    }

    fn save(&self) {
        crate::data::save_struct(FILE, &self);
    }

}