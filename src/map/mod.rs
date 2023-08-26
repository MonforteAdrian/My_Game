mod generation;
mod settings;
use bevy::log;

use crate::{loading::BlocksTextureAssets, AppState};
use bevy::math::Vec4Swizzles;
use bevy::{ecs::system::Resource, prelude::*};
use bevy_ecs_tilemap::prelude::*;

use generation::*;
use settings::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TilemapRenderSettings {
            render_chunk_size: UVec2::new(3, 1),
            y_sort: true,
        })
        .init_resource::<CursorPos>()
        .add_plugins(TilemapPlugin)
        .add_systems(
            OnEnter(AppState::MapCreation),
            (spawn_tilemap, apply_deferred).chain(),
        )
        .add_systems(
            Update,
            (
                update_cursor_pos,
                dehighlight_tile.run_if(not(any_with_component::<SelectedTile>())),
                highlight_tile,
            )
                .chain()
                .run_if(in_state(AppState::MapCreation)),
        );
    }
}

// State used for the current menu screen
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MapCreationState {
    MapSettings,
    MapGeneration,
    #[default]
    Disabled,
}

pub struct MapCreationSettings {}

const MAP_SIDE_LENGTH_X: u32 = 8;
const MAP_SIDE_LENGTH_Y: u32 = 8;

const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 32.0, y: 32.0 };

// Generates the initial tilemap, which is a square grid.
fn spawn_tilemap(mut commands: Commands, texture_assets: Res<BlocksTextureAssets>) {
    let map = generation::Map::new();
    let map_side = map.chunks.len() as f64;
    let map_side = map_side.sqrt() as usize;
    for (pos, chunk) in map.chunks.iter().enumerate() {
        log::info!("{}, {}", pos / map_side, pos % map_side);
        for z in chunk.tiles.iter() {
            for x in z.iter() {
                for _y in x.iter() {
                    // Here you change the tile
                }
            }
        }
    }

    // Size of the tile map in tiles.
    let map_size = TilemapSize {
        x: MAP_SIDE_LENGTH_X,
        y: MAP_SIDE_LENGTH_Y,
    };

    // This is the size of each individual tiles in pixels.
    let tile_size = TILE_SIZE;
    let grid_size = tile_size.into();
    let map_type = TilemapType::Isometric(IsoCoordSystem::Diamond);

    // To create a map we use the TileStorage component.
    // This component is a grid of tile entities and is used to help keep track of individual
    // tiles in the world. If you have multiple layers of tiles you would have a Tilemap2dStorage
    // component per layer.
    let mut tile_storage = TileStorage::empty(map_size);

    // Create a tilemap entity a little early
    // We want this entity early because we need to tell each tile which tilemap entity
    // it is associated with. This is done with the TilemapId component on each tile.
    let tilemap_entity = commands.spawn_empty().id();

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let texture_index = if (x + y) % 2 == 0 {
                TileTextureIndex(0)
            } else {
                TileTextureIndex(8)
            };
            let tile_entity = commands
                .spawn(TileBundle {
                    texture_index,
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    ..Default::default()
                })
                .id();
            // Here we let the tile storage component know what tiles we have.
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Vector(texture_assets.blocks_textures.values().cloned().collect()),
        tile_size,
        map_type,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });
}

//#[derive(Component)]
//struct TileLabel(Entity);
//
// Generates tile position labels of the form: `(tile_pos.x, tile_pos.y)`
//fn spawn_tile_labels(
//    mut commands: Commands,
//    tilemap_q: Query<(&Transform, &TilemapType, &TilemapGridSize, &TileStorage)>,
//    tile_q: Query<&mut TilePos>,
//    font_handle: Res<FontHandle>,
//) {
//    let text_style = TextStyle {
//        font: font_handle.clone(),
//        font_size: 20.0,
//        color: Color::BLACK,
//    };
//    let text_alignment = TextAlignment::Center;
//    for (map_transform, map_type, grid_size, tilemap_storage) in tilemap_q.iter() {
//        for tile_entity in tilemap_storage.iter().flatten() {
//            let tile_pos = tile_q.get(*tile_entity).unwrap();
//            let tile_center = tile_pos.center_in_world(grid_size, map_type).extend(1.0);
//            let transform = *map_transform * Transform::from_translation(tile_center);
//
//            let label_entity = commands
//                .spawn(Text2dBundle {
//                    text: Text::from_section(
//                        format!("{}, {}", tile_pos.x, tile_pos.y),
//                        text_style.clone(),
//                    )
//                    .with_alignment(text_alignment),
//                    transform,
//                    ..default()
//                })
//                .id();
//            commands
//                .entity(*tile_entity)
//                .insert(TileLabel(label_entity));
//        }
//    }
//}

#[derive(Resource)]
pub struct CursorPos(Vec2);
impl Default for CursorPos {
    fn default() -> Self {
        // Initialize the cursor pos at some far away place. It will get updated
        // correctly when the cursor moves.
        Self(Vec2::new(-1000.0, -1000.0))
    }
}

// We need to keep the cursor position updated based on any `CursorMoved` events.
pub fn update_cursor_pos(
    camera_q: Query<(&GlobalTransform, &Camera)>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut cursor_pos: ResMut<CursorPos>,
) {
    for cursor_moved in cursor_moved_events.iter() {
        // To get the mouse's world position, we have to transform its window position by
        // any transforms on the camera. This is done by projecting the cursor position into
        // camera space (world space).
        for (cam_t, cam) in camera_q.iter() {
            if let Some(pos) = cam.viewport_to_world_2d(cam_t, cursor_moved.position) {
                *cursor_pos = CursorPos(pos);
            }
        }
    }
}

#[derive(Component)]
struct PreselectedTile;

#[derive(Component)]
struct SelectedTile;

// This is where we check which tile the cursor is hovered over.
fn highlight_tile(
    mut commands: Commands,
    cursor_pos: Res<CursorPos>,
    tilemap_q: Query<(
        &TilemapSize,
        &TilemapGridSize,
        &TilemapType,
        &TileStorage,
        &Transform,
    )>,
    mut tile_texture_q: Query<&mut TileTextureIndex>,
    mouse_button_input: Res<Input<MouseButton>>,
) {
    for (map_size, grid_size, map_type, tile_storage, map_transform) in tilemap_q.iter() {
        // Grab the cursor position from the `Res<CursorPos>`
        let cursor_pos: Vec2 = cursor_pos.0;
        // We need to make sure that the cursor's world position is correct relative to the map
        // due to any map transformation.
        let cursor_in_map_pos: Vec2 = {
            // Extend the cursor_pos vec3 by 0.0 and 1.0
            let cursor_pos = Vec4::from((cursor_pos, 0.0, 1.0));
            let cursor_in_map_pos = map_transform.compute_matrix().inverse() * cursor_pos;
            cursor_in_map_pos.xy()
        };
        // Once we have a world position we can transform it into a possible tile position.
        if let Some(tile_pos) =
            TilePos::from_world_pos(&cursor_in_map_pos, map_size, grid_size, map_type)
        {
            // Highlight the relevant tile's label
            if let Some(tile_entity) = tile_storage.get(&tile_pos) {
                if let Ok(mut texture) = tile_texture_q.get_mut(tile_entity) {
                    texture.0 = 16;
                    commands.entity(tile_entity).insert(PreselectedTile);
                    if mouse_button_input.pressed(MouseButton::Left) {
                        commands
                            .entity(tile_entity)
                            .remove::<PreselectedTile>()
                            .insert(SelectedTile);
                    }
                }
            }
        }
    }
}

fn dehighlight_tile(
    mut commands: Commands,
    preselected_tiles_q: Query<Entity, With<PreselectedTile>>,
    mut tile_texture_q: Query<&mut TileTextureIndex>,
) {
    // Un-highlight any previously highlighted tile labels.
    for highlighted_tile_entity in preselected_tiles_q.iter() {
        if let Ok(mut texture) = tile_texture_q.get_mut(highlighted_tile_entity) {
            *texture = TileTextureIndex(0);
            commands
                .entity(highlighted_tile_entity)
                .remove::<PreselectedTile>();
        }
    }
}
