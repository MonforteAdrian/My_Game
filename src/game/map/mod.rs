use bevy::prelude::*;
use super::GameState;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(GameState::MapCreation).with_system(startup));
    }
}

const SIZE_MAP: u32 = 16;

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
}
