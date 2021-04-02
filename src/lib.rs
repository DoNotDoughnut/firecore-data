use std::path::{Path, PathBuf};
use directories_next::ProjectDirs;
use error::DataError;
use macroquad::prelude::{warn, info};


pub use macroquad::prelude::collections::storage::{get, get_mut};
pub use firecore_saves::*;

pub mod configuration;

pub mod error;
pub mod reload;

pub async fn store() {
    store_data::<configuration::Configuration>("configuration").await;
    store_data::<player::PlayerSaves>("player saves").await;
}

async fn store_data<D: PersistantData + Sized + 'static>(name: &str) {
    match load::<D>().await {
        Ok(data) => macroquad::prelude::collections::storage::store(data),
        Err(err) => {
            warn!("Could not load {} with error {}", name, err);
            info!("Saving a new {} file!", name);
            let data = D::default();
            if let Err(err) = save(&data) {
                warn!("Could not save new {} with error {}", name, err);
            }
            macroquad::prelude::collections::storage::store(data);
        }
    }
}

pub async fn load<D: PersistantData + Sized>() -> Result<D, DataError> {
    let path = Path::new(D::path());
    #[cfg(not(target_arch = "wasm32"))]
    let string = {
        match crate::directory() {
            Ok(dir) => Ok(
                (
                    *String::from_utf8_lossy(
                        &macroquad::prelude::load_file(
                            &*dir.join(path).to_string_lossy()
                        ).await?
                    )
                ).to_owned()
            ),
            Err(err) => Err(err),
        }      
    }?;
    #[cfg(target_arch = "wasm32")]
    let string = {
        match path.file_stem() {
            Some(fname) => Ok(miniquad_cookie::get_cookie(&fname.to_string_lossy())),
            None => Err(DataError::NoFileName),
        }
    }?;
    let data: D = ron::from_str(&string).map_err(|error| DataError::Deserialize(D::path(), error))?;
    Ok(data)
}

pub fn save<D: PersistantData>(data: &D) -> Result<(), DataError> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        if let Ok(dir) = crate::directory() {

            let path = dir.join(D::path());

            if let Some(parent) = path.parent() {
                if !parent.exists() {
                    std::fs::create_dir_all(&parent)?;
                }
            }

            let mut file = std::fs::File::create(&path)?;

            let string = ron::ser::to_string_pretty(data, ron::ser::PrettyConfig::default())?;

            std::io::Write::write_all(&mut file, string.as_bytes())?;

            Ok(())
        } else {
            Err(DataError::NoDirectory)
        }

    }

    #[cfg(target_arch = "wasm32")]
    {
        if let Some(fname) = Path::new(D::path()).file_stem() {
            match ron::to_string(&data) {
                Ok(string) => {
                    miniquad_cookie::set_cookie(&fname.to_string_lossy(), &string);
                    Ok(())
                },
                Err(err) => Err(DataError::Serialize(err)),
            }
        } else {
            Err(DataError::NoDirectory)
        }
    }
}

pub async fn reload<D: reload::Reloadable + Sized>(data: &mut D) -> Result<(), DataError> {
    *data = load::<D>().await?;
    data.on_reload();
    Ok(())
}

pub fn directory() -> Result<PathBuf, DataError> {

    let data_dir = ProjectDirs::from("net", "rhysholloway", "pokemon-firered-clone");

    let path = data_dir.as_ref().map(|dir| PathBuf::from(dir.data_dir()));
    if let Some(real_path) = path.as_ref() {
        if let Ok(metadata) = std::fs::metadata(real_path) {
            if !metadata.permissions().readonly() {
                return path.ok_or(DataError::ReadOnly);
            }
        } else {
            if !real_path.exists() {
                if let Ok(()) = std::fs::create_dir_all(real_path) {
                    return directory();
                }
            }
        }
    }
    std::env::current_dir().map_err(|err| DataError::IOError(err))
}