pub mod components;
pub mod resources;

use bevy::log;
use bevy::prelude::*;
use resources::tile_map::TileMap;

pub struct MyPlugin;

impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::create_board);
        log::info!("Loaded Board Plugin");
    }
}

impl MyPlugin {
    pub fn create_board() {
        let mut tile_map = TileMap::empty(20,20);
        #[cfg(feature = "debug")]
        log::info!("{}", tile_map.console_output());
    }
}
