// TODO this is part of the screens for generating map should be moved out
mod generation;
mod settings;

mod chunks;
mod layers;
mod layout;
mod tiletype;
pub use layout::Layout;
mod algorithms;
mod matrix;
pub use algorithms::*;
mod position;
pub use position::*;
mod direction;
pub use direction::{DirectionWay, EdgeDirection, VertexDirection};

use crate::{GameState, WorldCreationState};
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
