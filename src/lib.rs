use std::path::PathBuf;

use directories_next::ProjectDirs;

pub use macroquad::prelude::collections::storage::{get, get_mut};
pub use firecore_data_lib::*;

pub mod data;
pub mod error;

pub async fn load() {

    use macroquad::prelude::collections::storage::store;
    use data::PersistantDataLocation;

    let config = configuration::Configuration::load_from_file().await;

    store(config);

    let saves = player::list::PlayerSaves::load_from_file().await;

    store(saves);

}

pub fn get_save_dir() -> Result<PathBuf, error::Error> {

    let data_dir = ProjectDirs::from("net", "rhysholloway", "pokemon-firered-clone");

    let path = data_dir.as_ref().map(|dir| PathBuf::from(dir.data_dir()));
    if let Some(real_path) = path.as_ref() {
        if let Ok(metadata) = std::fs::metadata(real_path) {
            if !metadata.permissions().readonly() {
                return path.ok_or(error::Error::ReadOnly);
            }
        } else {
            if !real_path.exists() {
                if let Ok(()) = std::fs::create_dir_all(real_path) {
                    return get_save_dir();
                }
            }
        }
    }
    std::env::current_dir().map_err(|err| error::Error::IOError(err))
}