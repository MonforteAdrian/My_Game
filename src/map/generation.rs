use super::*;
use crate::{
    GameState, SpawnType, WorldCreationState, WorldMap,
    map::chunks::{get_sorted_tiles, split_map},
    spawner::SpawnEntity,
};
use bevy::prelude::*;

// TODO convert this to resources in map creation
#[allow(dead_code)]
const CHUNK_SIZE: Vec2 = Vec2::new(
    32.0 * CHUNK_DIMENSIONS.0 as f32,
    32.0 * (CHUNK_DIMENSIONS.1 as f32 / 4.0),
);

pub struct MapGenerationPlugin;

impl Plugin for MapGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(WorldCreationState::MapGeneration), map_generation_startup)
            .add_systems(
                OnExit(WorldCreationState::MapGeneration),
                crate::despawn_screen::<MapGenerationScreen>,
            );
    }
}

#[derive(Component)]
struct MapGenerationScreen;

fn map_generation_startup(
    mut map_creation_state: ResMut<NextState<WorldCreationState>>,
    mut app_state: ResMut<NextState<GameState>>,
    mut spawn_event: EventWriter<SpawnEntity>,
    //window: Query<&mut Window>,
    world_map: Res<WorldMap>,
) {
    let noise_map = world_map.generate();

    // TODO to be used later in conjuction with noise map
    //let window = window.single();
    //let chunks_wide = (window.width() / CHUNK_SIZE.x).ceil();
    //let chunks_height = (window.height() / CHUNK_SIZE.y).ceil();
    //let cols = (chunks_wide / 2.0) as i32;
    //let rows = (chunks_height / 2.0) as i32;

    let chunks = split_map(&noise_map);
    get_sorted_tiles(&chunks).into_iter().for_each(|tile| {
        // Checkboard pattern
        //let (chunk_x, chunk_y) = tile.chunk();
        //let name = if (chunk_x + chunk_y) % 2 == 0 { "GrassBlock".to_string() } else { "StoneBlock".to_string() };

        // z levels
        let material = match tile.pos.z {
            ..1 => "Stone".to_string(),
            1..2 => "Sand".to_string(),
            _ => "Grass".to_string(),
        };
        let style = match tile.tile_type {
            TileType::Block => "Block".to_string(),
            TileType::Floor => "Floor".to_string(),
        };
        let name = format!("{}{}", material, style).to_string();
        spawn_event.write(SpawnEntity {
            name,
            pos: SpawnType::AtPosition {
                x: tile.pos.x,
                y: tile.pos.y,
                z: tile.pos.z,
            },
        });
    });

    let x = 0;
    let y = 0;
    let z = chunks
        .get(&position(x, y, 0).chunk())
        .expect("extected to find a chunk")
        .find_top_layer(&position(x, y, 0));
    spawn_event.write(SpawnEntity {
        name: "Dummy".to_string(),
        pos: SpawnType::AtPosition { x, y, z },
    });
    //let x = 20;
    //let y = 20;
    //let z = chunks
    //    .get(&position(x, y, 0).chunk())
    //    .expect("extected to find a chunk")
    //    .find_top_layer(&position(x, y, 0));
    //spawn_event.write(SpawnEntity {
    //    name: "BadDummy".to_string(),
    //    pos: SpawnType::AtPosition { x, y, z },
    //});
    let x = 10;
    let y = 10;
    let z = chunks
        .get(&position(x, y, 0).chunk())
        .expect("extected to find a chunk")
        .find_top_layer(&position(x, y, 0));
    spawn_event.write(SpawnEntity {
        name: "Heart".to_string(),
        pos: SpawnType::AtPosition { x, y, z },
    });
    let x = 15;
    let y = 5;
    let z = chunks
        .get(&position(x, y, 0).chunk())
        .expect("extected to find a chunk")
        .find_top_layer(&position(x, y, 0));
    spawn_event.write(SpawnEntity {
        name: "RustySword".to_string(),
        pos: SpawnType::AtPosition { x, y, z },
    });
    let x = 5;
    let y = 5;
    let z = chunks
        .get(&position(x, y, 0).chunk())
        .expect("extected to find a chunk")
        .find_top_layer(&position(x, y, 0));
    spawn_event.write(SpawnEntity {
        name: "TreeWithFruit".to_string(),
        pos: SpawnType::AtPosition { x, y, z },
    });
    let x = 5;
    let y = 15;
    let z = chunks
        .get(&position(x, y, 0).chunk())
        .expect("extected to find a chunk")
        .find_top_layer(&position(x, y, 0));
    spawn_event.write(SpawnEntity {
        name: "Tree".to_string(),
        pos: SpawnType::AtPosition { x, y, z },
    });

    // TODO this should be moved out
    map_creation_state.set(WorldCreationState::Disabled);
    app_state.set(GameState::InGame);
}
