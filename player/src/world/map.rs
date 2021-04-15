use serde::{Deserialize, Serialize};
use ahash::{AHashSet as HashSet, AHashMap as HashMap};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct MapData {

    #[serde(default)]
    pub battled: HashSet<u8>,

    #[serde(default)]
    pub npcs: HashMap<u8, bool>, // npc states, active / not active

}