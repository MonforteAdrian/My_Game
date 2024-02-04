mod generation;
mod settings;

use crate::AppState;
use bevy::prelude::*;

use settings::MapSettingsBundle;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<MapCreationState>()
            .add_systems(OnEnter(AppState::MapCreation), map_setup)
            .insert_resource(MapSettingsBundle::build());
    }
}

// State used for the current menu screen
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MapCreationState {
    MapSettings,
    MapGeneration,
    #[default]
    Disabled,
}

// Generates the initial tilemap, which is a square grid.
fn map_setup(
    mut map_creation_state: ResMut<NextState<MapCreationState>>,
    map_settings_bundle: Res<MapSettingsBundle>,
) {
    println!("Started the Creation of the map");
    println!("{:?}", map_settings_bundle);
    map_creation_state.set(MapCreationState::MapSettings);
}
