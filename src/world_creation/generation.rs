use super::*;
use crate::despawn_screen;
use bevy::{prelude::*, utils::HashMap};
use glam::uvec2;
use isometric::*;
use std::ops::Neg;

// TODO convert this to resources in map creation
const TILE_SIZE: Vec2 = Vec2::splat(64.0);
const CHUNK_SIZE: Vec2 = Vec2::new(
    TILE_SIZE.x * CHUNK_DIMENSIONS.0 as f32,
    TILE_SIZE.y * (CHUNK_DIMENSIONS.1 as f32 / 4.0),
);

pub struct MapGenerationPlugin;

impl Plugin for MapGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(WorldCreationState::MapGeneration),
            (get_window_to_chunks, map_generation_startup),
        )
        .add_systems(
            OnExit(WorldCreationState::MapGeneration),
            despawn_screen::<MapGenerationScreen>,
        );
    }
}

#[derive(Component)]
struct MapGenerationScreen;

#[derive(Debug, Resource)]
struct IsoGrid {
    pub entities: HashMap<Iso, Entity>,
    pub layout: IsoLayout,
}

// TODO call this function on window resize or on tile size change
fn get_window_to_chunks(window: Query<&mut Window>) {
    // We get the window dimensions and calculate the numnber of tiles that would fit width and height adding a bit of room outside
    let window = window.single();
    let chunks_wide = (window.width() / CHUNK_SIZE.x).ceil();
    let chunks_height = (window.height() / CHUNK_SIZE.y).ceil();
    // TODO create a resource in which to put this values
    let cols = (chunks_wide / 2.0) as i32;
    let rows = (chunks_height / 2.0) as i32;
}

fn map_generation_startup(
    mut commands: Commands,
    // TODO use the individual textures
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
    mut map_creation_state: ResMut<NextState<WorldCreationState>>,
    mut app_state: ResMut<NextState<GameState>>,
    // TODO remove this
    window: Query<&mut Window>,
) {
    // TODO remove this once you fix the get_window_to_chunks
    let window = window.single();
    let chunks_wide = (window.width() / CHUNK_SIZE.x).ceil();
    let chunks_height = (window.height() / CHUNK_SIZE.y).ceil();
    let cols = (chunks_wide / 2.0) as i32;
    let rows = (chunks_height / 2.0) as i32;

    let texture = asset_server.load("sprites/the_four_horsemen_of_the_apocalypse.png");
    let atlas_layout = TextureAtlasLayout::from_grid(uvec2(32, 32), 2, 2, None, None);
    let atlas_layout = atlas_layouts.add(atlas_layout);

    let offset_layers = TILE_SIZE.y / 2.0 * 3.0;
    let offset_center_tile = TILE_SIZE.y / 4.0;
    // Create the Layout
    let layout = IsoLayout {
        tile_size: TILE_SIZE,
        origin: Vec3::new(0., -(offset_layers + offset_center_tile), 0.),
        ..default()
    };

    // Generate the tiles and spawn them
    let chunks = mapping::generate_mesh_of_chunks(cols, cols.neg(), rows, rows.neg());
    let entities = mapping::get_sorted_tiles(chunks)
        .into_iter()
        .map(|tile| {
            let pos = layout.tile_to_world_pos(tile);
            let index = 0;
            let entity = commands
                .spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(TILE_SIZE),
                            ..default()
                        },
                        texture: texture.clone(),
                        transform: Transform::from_xyz(pos.x, pos.y, pos.z),
                        ..default()
                    },
                    TextureAtlas {
                        index,
                        layout: atlas_layout.clone(),
                    },
                ))
                .id();
            (tile, entity)
        })
        .collect();
    commands.insert_resource(IsoGrid { entities, layout });

    // TODO this should be moved out
    map_creation_state.set(WorldCreationState::Disabled);
    app_state.set(GameState::InGame);
}
