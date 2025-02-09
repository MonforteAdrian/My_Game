use crate::GameState;
use bevy::prelude::{App, OnEnter, Plugin, ResMut};
use std::fs;

mod tile_bundle;
use tile_bundle::*;
mod creature_bundle;
use creature_bundle::*;
mod item_bundle;
use item_bundle::*;

mod rawmaster;
pub use rawmaster::*;

const TILES_FILE: &str = "./data/tiles/tiles.ron";
const CREATURES_FILE: &str = "./data/creatures/creatures.ron";
const ITEMS_FILE: &str = "./data/items/items.ron";

pub struct RawsPlugin;

impl Plugin for RawsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RawMaster>()
            .add_systems(OnEnter(GameState::InMapCreation), load_creatures_from_ron);
    }
}

fn load_creatures_from_ron(mut raw_master: ResMut<RawMaster>) {
    let ron_tiles = fs::read_to_string(TILES_FILE).expect("Unable to read the raws file");
    raw_master.raws.tiles = ron::from_str(&ron_tiles).expect("Failed to deserialize from RON");

    let ron_creatures = fs::read_to_string(CREATURES_FILE).expect("Unable to read the raws file");
    raw_master.raws.creatures = ron::from_str(&ron_creatures).expect("Failed to deserialize from RON");

    let ron_tiles = fs::read_to_string(ITEMS_FILE).expect("Unable to read the raws file");
    raw_master.raws.items = ron::from_str(&ron_tiles).expect("Failed to deserialize from RON");

    raw_master.load();
}
