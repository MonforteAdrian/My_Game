use bevy::prelude::*;
use super::GameState;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(startup.in_schedule(OnEnter(GameState::MapCreation)));
    }
}

const SIZE_MAP: u32 = 16;

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
}
