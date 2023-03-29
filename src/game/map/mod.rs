use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use super::GameState;

mod helpers;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(TilemapRenderSettings {
                render_chunk_size: UVec2::new(3, 1),
            })
            .add_plugin(TilemapPlugin)
            .add_plugin(helpers::tiled::TiledMapPlugin)
            .add_system(startup.in_schedule(OnEnter(GameState::MapCreation)));
    }
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map_handle: Handle<helpers::tiled::TiledMap> = asset_server.load("iso_map.tmx");

    commands.spawn(helpers::tiled::TiledMapBundle {
        tiled_map: map_handle,
        ..Default::default()
    });
}
