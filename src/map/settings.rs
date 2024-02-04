use super::*;
use crate::despawn_screen;
use bevy::prelude::*;

pub struct MapSettingsPlugin;

impl Plugin for MapSettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MapCreationState::MapSettings), map_settings_startup)
            .add_systems(
                OnExit(MapCreationState::MapSettings),
                despawn_screen::<MapSettingsScreen>,
            );
    }
}

#[derive(Component)]
struct MapSettingsScreen;

fn map_settings_startup(mut map_creation_state: ResMut<NextState<MapCreationState>>) {
    map_creation_state.set(MapCreationState::MapGeneration);
}

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone)]
pub struct MapSettingsBundle {
    size: (i32, i32),
    name: String,
    seed: i32,
}

impl MapSettingsBundle {
    pub fn build() -> Self {
        MapSettingsBundle {
            ..Default::default()
        }
    }
}

impl Default for MapSettingsBundle {
    fn default() -> Self {
        MapSettingsBundle {
            size: (12, 12),
            name: "Test".to_string(),
            seed: 0,
        }
    }
}
