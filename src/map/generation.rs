use super::*;
use crate::{
    map::chunks::{generate_mesh_of_chunks, get_sorted_tiles},
    spawner::SpawnEntity,
    GameState, SpawnType, WorldCreationState, WorldMap,
};
use bevy::prelude::*;
use std::ops::Neg;

// TODO convert this to resources in map creation
const CHUNK_SIZE: Vec2 = Vec2::new(32.0 * CHUNK_DIMENSIONS.0 as f32, 32.0 * (CHUNK_DIMENSIONS.1 as f32 / 4.0));

pub struct MapGenerationPlugin;

impl Plugin for MapGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(WorldCreationState::MapGeneration), map_generation_startup)
            .add_systems(OnExit(WorldCreationState::MapGeneration), crate::despawn_screen::<MapGenerationScreen>);
    }
}

#[derive(Component)]
struct MapGenerationScreen;

fn map_generation_startup(
    mut map_creation_state: ResMut<NextState<WorldCreationState>>,
    mut app_state: ResMut<NextState<GameState>>,
    mut spawn_event: EventWriter<SpawnEntity>,
    window: Query<&mut Window>,
    mut world_map: ResMut<WorldMap>,
) {
    world_map.generate();
    world_map.x_bounds = (0.0, 0.5);
    world_map.y_bounds = (0.0, 0.5);
    world_map.generate();

    // TODO remove this once you fix the get_window_to_chunks
    let window = window.single();
    let chunks_wide = (window.width() / CHUNK_SIZE.x).ceil();
    let chunks_height = (window.height() / CHUNK_SIZE.y).ceil();
    let cols = (chunks_wide / 2.0) as i32;
    let rows = (chunks_height / 2.0) as i32;

    // Generate the tiles and spawn them
    //let chunks = generate_mesh_of_chunks(cols, cols.neg(), rows, rows.neg());
    // Not full screen to see depth
    let chunks = generate_mesh_of_chunks(1, -1, 1, -1);
    get_sorted_tiles(chunks).into_iter().enumerate().for_each(|(i, tile)| {
        let name = if i % 11 == 0 { "GrassBlock".to_string() } else { "StoneFloor".to_string() };
        spawn_event.send(SpawnEntity { name, pos: SpawnType::AtPosition { x: tile.x, y: tile.y, z: tile.z } });
        if i == 15 {
            spawn_event.send(SpawnEntity {
                name: "Dummy".to_string(),
                pos: SpawnType::AtPosition { x: tile.x, y: tile.y, z: tile.z },
            });
        }
        if i == 18 {
            spawn_event.send(SpawnEntity {
                name: "BadDummy".to_string(),
                pos: SpawnType::AtPosition { x: tile.x, y: tile.y, z: tile.z },
            });
        }
        //if i == 19 {
        //    spawn_event.send(SpawnEntity {
        //        name: "Heart".to_string(),
        //        pos: SpawnType::AtPosition { x: tile.x, y: tile.y, z: tile.z },
        //    });
        //}
    });

    // TODO this should be moved out
    map_creation_state.set(WorldCreationState::Disabled);
    app_state.set(GameState::InGame);
}
