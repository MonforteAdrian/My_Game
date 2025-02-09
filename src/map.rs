// TODO this is part of the screens for generating map should be moved out
mod generation;
mod settings;

mod chunks;
pub use chunks::*;
mod layers;
pub(crate) use layers::TileData;
mod layout;
pub(crate) use layout::TILE_SIZE;
mod tiletype;
pub use layout::Layout;
pub(crate) use tiletype::TileType;
mod matrix;
mod position;
pub(crate) use position::*;

use crate::{GameState, WorldCreationState};
use bevy::prelude::*;

use generation::MapGenerationPlugin;
use layers::Layer;
use settings::MapSettingsPlugin;

pub struct WorldCreationPlugin;

impl Plugin for WorldCreationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MapSettingsPlugin)
            .add_plugins(MapGenerationPlugin)
            .add_systems(OnEnter(GameState::InMapCreation), map_setup);
    }
}

fn map_setup(mut map_creation_state: ResMut<NextState<WorldCreationState>>) {
    map_creation_state.set(WorldCreationState::MapSettings);
}
