use super::*;
use crate::despawn_screen;
use bevy::prelude::*;

pub struct MapGenerationPlugin;

impl Plugin for MapGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(MapCreationState::MapGeneration),
            map_generation_startup,
        )
        .add_systems(
            OnExit(MapCreationState::MapGeneration),
            despawn_screen::<MapGenerationScreen>,
        );
    }
}

#[derive(Component)]
struct MapGenerationScreen;

fn map_generation_startup(
    mut map_creation_state: ResMut<NextState<MapCreationState>>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    map_creation_state.set(MapCreationState::Disabled);
    app_state.set(AppState::Game);
}

// We generate the complete map at the start dwarf fortress style.
// There should be a vector/array of chunks depending on the size of the map choosed before
// The position of the array in the map is calculated by
//  the index in the vector / sqrt(len) to get the x and index in the vector % 4 to get the y
// Every chunk should have a 3d vector/array x * y * z fixed size
const CHUNK_SIZE: usize = 16;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Empty,
    Grass,
    Sand,
    Stone,
}

#[derive(Clone)]
pub struct Chunk {
    pub tiles: [[[TileType; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
}

pub struct Map {
    pub chunks: Vec<Chunk>,
}

impl Chunk {
    fn new() -> Self {
        Self {
            tiles: [[[TileType::Grass; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
        }
    }

    pub fn render(&self) {}
}

impl Map {
    pub fn new(map_settings: Res<MapSettingsBundle>) -> Self {
        Self {
            chunks: vec![Chunk::new(); 25],
        }
    }
}
