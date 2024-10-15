use crate::prelude::*;
use bevy::prelude::*;

pub struct MapSettingsPlugin;

impl Plugin for MapSettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(WorldCreationState::MapSettings),
            map_settings_startup,
        )
        .add_systems(
            OnExit(WorldCreationState::MapSettings),
            despawn_screen::<MapSettingsScreen>,
        );
    }
}

#[derive(Component)]
struct MapSettingsScreen;

fn map_settings_startup(mut map_creation_state: ResMut<NextState<WorldCreationState>>) {
    map_creation_state.set(WorldCreationState::MapGeneration);
}
