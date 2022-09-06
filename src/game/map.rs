use std::time::Duration;

use bevy::{prelude::*, utils::HashMap};

use super::GameState;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        info!("Hola");
        app
            .add_system_set(SystemSet::on_enter(GameState::MapCreation).with_system(create_map));
    }
}

fn create_map(mut commands: Commands, asset_server: Res<AssetServer>) {
        info!("Hola");
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("sprites/cube_green.png"),
        ..default()
    });
}
