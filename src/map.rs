mod generation;
mod settings;
mod tiletype;

// This is from iso
mod chunks;
mod layers;
mod layout;
mod matrix;
mod position;
pub use position::Position;
mod pathfinding;
pub use pathfinding::a_star;

use crate::prelude::*;
use bevy::prelude::*;

use generation::MapGenerationPlugin;
use layers::Layer;
use settings::MapSettingsPlugin;

/// (columns, rows, layers)
pub const CHUNK_DIMENSIONS: (i32, i32, i32) = (12, 12, 4);

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
