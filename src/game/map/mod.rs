use crate::prelude::*;
use bevy::math::Vec4Swizzles;
use bevy::{ecs::system::Resource, prelude::*};
use bevy_ecs_tilemap::prelude::*;

use super::GameState;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TilemapRenderSettings {
            render_chunk_size: UVec2::new(3, 1),
            y_sort: true,
        })
        .init_resource::<CursorPos>()
        .init_resource::<TilesHandle>()
        .init_resource::<FontHandle>()
        .add_plugin(TilemapPlugin)
        .add_systems(
            (spawn_tilemap, apply_system_buffers)
                .chain()
                .in_schedule(OnEnter(GameState::MapCreation)),
        )
        .add_systems(
            (
                update_cursor_pos,
                dehighlight_tile.run_if(not(any_with_component::<SelectedTile>())),
                highlight_tile,
            )
                .chain(),
        );
    }
}

const MAP_SIDE_LENGTH_X: u32 = 8;
const MAP_SIDE_LENGTH_Y: u32 = 8;

const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 32.0, y: 32.0 };
const GRID_SIZE: TilemapGridSize = TilemapGridSize { x: 32.0, y: 32.0 };

#[derive(Deref, Resource)]
pub struct TilesHandle(Vec<HandleUntyped>);

fn load_assets(mut commands: Commands, server: Res<AssetServer>) {
    if let Ok(handles) = server.load_folder("extra") {
        commands.insert_resource(TilesHandle(handles));
    }
}

#[derive(Deref, Resource)]
pub struct FontHandle(Handle<Font>);

impl FromWorld for TilesHandle {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self(asset_server.load_folder("sprites/blocks").unwrap())
    }
}
impl FromWorld for FontHandle {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self(asset_server.load("fonts/FiraSans-Bold.ttf"))
    }
}

// Generates the initial tilemap, which is a square grid.
fn spawn_tilemap(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map_size = TilemapSize {
        x: MAP_SIDE_LENGTH_X,
        y: MAP_SIDE_LENGTH_Y,
    };
    let map_size2 = TilemapSize {
        x: MAP_SIDE_LENGTH_X / 4,
        y: MAP_SIDE_LENGTH_Y / 4,
    };
    let tile_size = TILE_SIZE;
    let grid_size = GRID_SIZE;
    let map_type = TilemapType::Isometric(IsoCoordSystem::Diamond);

    //Layer 1
    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();
    let tilemap_id = TilemapId(tilemap_entity);

    fill_tilemap(
        TileTextureIndex(0),
        map_size,
        tilemap_id,
        &mut commands,
        &mut tile_storage,
    );

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(asset_server.get_handle("sprites/blocks/grass_block.png")),
        tile_size,
        map_type,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });

    //Layer 2
    let mut tile_storage = TileStorage::empty(map_size2);
    let tilemap_entity = commands.spawn_empty().id();
    let tilemap_id = TilemapId(tilemap_entity);

    fill_tilemap(
        TileTextureIndex(0),
        map_size2,
        tilemap_id,
        &mut commands,
        &mut tile_storage,
    );

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        size: map_size2,
        storage: tile_storage,
        texture: TilemapTexture::Single(asset_server.get_handle("sprites/blocks/stone_block.png")),
        tile_size,
        map_type,
        transform: get_tilemap_center_transform(&map_size2, &grid_size, &map_type, 1.0)
            * Transform::from_xyz(0.0, 32.0, 0.0),
        ..Default::default()
    });
}

#[derive(Component)]
struct TileLabel(Entity);

// Generates tile position labels of the form: `(tile_pos.x, tile_pos.y)`
fn spawn_tile_labels(
    mut commands: Commands,
    tilemap_q: Query<(&Transform, &TilemapType, &TilemapGridSize, &TileStorage)>,
    tile_q: Query<&mut TilePos>,
    font_handle: Res<FontHandle>,
) {
    let text_style = TextStyle {
        font: font_handle.clone(),
        font_size: 20.0,
        color: Color::BLACK,
    };
    let text_alignment = TextAlignment::Center;
    for (map_transform, map_type, grid_size, tilemap_storage) in tilemap_q.iter() {
        for tile_entity in tilemap_storage.iter().flatten() {
            let tile_pos = tile_q.get(*tile_entity).unwrap();
            let tile_center = tile_pos.center_in_world(grid_size, map_type).extend(1.0);
            let transform = *map_transform * Transform::from_translation(tile_center);

            let label_entity = commands
                .spawn(Text2dBundle {
                    text: Text::from_section(
                        format!("{}, {}", tile_pos.x, tile_pos.y),
                        text_style.clone(),
                    )
                    .with_alignment(text_alignment),
                    transform,
                    ..default()
                })
                .id();
            commands
                .entity(*tile_entity)
                .insert(TileLabel(label_entity));
        }
    }
}

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
    mut tile_texture_q: Query<&mut TilemapTexture>,
    mouse_button_input: Res<Input<MouseButton>>,
    asset_server: Res<AssetServer>,
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
                    *texture = TilemapTexture::Single(
                        asset_server.get_handle("sprites/blocks/sand_block.png"),
                    );
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
            texture.0 = 0;
            commands
                .entity(highlighted_tile_entity)
                .remove::<PreselectedTile>();
        }
    }
}
