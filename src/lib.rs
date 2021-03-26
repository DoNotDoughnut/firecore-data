use std::path::PathBuf;

use directories_next::ProjectDirs;

pub mod data;
pub mod error;

pub mod configuration;
pub mod saves;

lazy_static::lazy_static! {
	pub static ref DATA_DIR: Option<ProjectDirs> = ProjectDirs::from("net", "rhysholloway", "pokemon-firered-clone");
}

pub async fn load() {

    use macroquad::prelude::collections::storage;
    use data::PersistantDataLocation;

    let config = configuration::Configuration::load_from_file().await;

    storage::store(config);

    let saves = firecore_data_lib::player::list::PlayerSaves::load_from_file().await;

    storage::store(saves);

}

pub fn get_save_dir() -> Result<PathBuf, error::Error> {
    let path = DATA_DIR.as_ref().map(|dir| PathBuf::from(dir.data_dir()));
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