mod generation;
mod settings;

use crate::prelude::*;
use bevy::prelude::*;

use generation::MapGenerationPlugin;
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
