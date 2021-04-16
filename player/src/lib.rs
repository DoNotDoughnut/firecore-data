use firecore_pokedex::pokemon::party::PokemonParty;
use serde::{Deserialize, Serialize};
use firecore_util::{
	GlobalPosition, Location, Position, Coordinate,
	tinystr::TinyStr16,
};

use world::WorldStatus;


mod list;
pub mod world;

pub use list::PlayerSaves;


#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerSave {

	pub name: String,

	#[serde(default = "default_location")]
	pub location: Location,

	#[serde(default)]
	pub party: PokemonParty,

	#[serde(default)]
	pub worth: usize,

	#[serde(default)]
	pub world_status: WorldStatus,

}

impl PlayerSave {

	pub fn new(name: &str) -> Self {
		Self {
			name: name.to_owned(),
			..Default::default()
		}
	}

	pub fn has_battled(&self, map: &String, npc: &u8) -> bool {
		self.world_status.map_data.get(map).map(|map| map.battled.contains(npc)).unwrap_or(false)
	}
	
}

impl Default for PlayerSave {
    fn default() -> Self {
		Self {
			name: default_name(),
			party: PokemonParty::default(),
			location: default_location(),
		    worth: 0,
		    world_status: WorldStatus::default(),
		}
	}

}

pub fn default_name() -> String {
	"Red".to_owned()
}

pub fn default_location() -> Location {
	Location {
		map: Some(default_map()),
		index: default_index(),
		position: GlobalPosition {
			local: Position {
				coords: Coordinate {
					x: 6,
					y: 6,
				},
				..Default::default()
			},
			..Default::default()
		}		
	}
}

pub fn default_map() -> TinyStr16 {
	"pallet_houses".parse().expect("Could not get map")
}

pub fn default_index() -> TinyStr16 {
	"player_room".parse().expect("Could not get map index")
}