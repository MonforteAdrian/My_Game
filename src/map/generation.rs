use super::*;
use crate::{
    components::{Mob, PathfindingSteps},
    despawn_screen,
    map::chunks::{generate_mesh_of_chunks, get_sorted_tiles},
    map::layout::Layout,
};
use bevy::{log, prelude::*, utils::HashMap, utils::HashSet, window::PrimaryWindow};
use std::{collections::VecDeque, ops::Neg};

// TODO convert this to resources in map creation
const TILE_SIZE: Vec2 = Vec2::splat(64.0);
const CHUNK_SIZE: Vec2 = Vec2::new(
    TILE_SIZE.x * CHUNK_DIMENSIONS.0 as f32,
    TILE_SIZE.y * (CHUNK_DIMENSIONS.1 as f32 / 4.0),
);

pub struct MapGenerationPlugin;

impl Plugin for MapGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedTile>()
            .init_resource::<MoveTimer>()
            .add_systems(
                OnEnter(WorldCreationState::MapGeneration),
                (get_window_to_chunks, map_generation_startup),
            )
            .add_systems(
                Update,
                (handle_input, hover_highlight, move_mob)
                    .run_if(in_state(WorldCreationState::MapGeneration)),
            )
            .add_systems(
                OnExit(WorldCreationState::MapGeneration),
                despawn_screen::<MapGenerationScreen>,
            );
    }
}
#[derive(Resource)]
struct MoveTimer(Timer);

impl MoveTimer {
    pub fn new() -> Self {
        Self(Timer::from_seconds(0.2, TimerMode::Repeating))
    }
}

impl Default for MoveTimer {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Component)]
struct MapGenerationScreen;

#[derive(Debug, Default, Resource)]
struct SelectedTile {
    pub selected: Option<Position>,
}

#[derive(Debug, Resource)]
struct IsoGrid {
    pub entities: HashMap<Position, Entity>,
    pub layout: Layout,
    pub blocked_coords: HashSet<Position>,
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
    let atlas_layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 2, 4, None, None);
    let atlas_layout = atlas_layouts.add(atlas_layout);

    let offset_layers = TILE_SIZE.y / 2.0 * 3.0;
    let offset_center_tile = TILE_SIZE.y / 4.0;
    // Create the Layout
    let layout = Layout {
        tile_size: TILE_SIZE,
        origin: Vec3::new(0., -(offset_layers + offset_center_tile), 0.),
        ..default()
    };

    let mut blocked_coords = HashSet::new();
    // Generate the tiles and spawn them
    //let chunks = generate_mesh_of_chunks(cols, cols.neg(), rows, rows.neg());
    // Not full screen to see depth
    let chunks = generate_mesh_of_chunks(cols / 2, cols.neg() / 2, rows / 2, rows.neg() / 2);
    let entities = get_sorted_tiles(chunks)
        .into_iter()
        .enumerate()
        .map(|(i, tile)| {
            let pos = layout.tile_to_world_pos(tile);
            let index = if i % 11 == 0 {
                blocked_coords.insert(tile);
                1
            } else {
                4
            };
            //let index = 0;
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
                .with_children(|b| {
                    if tile.z == layout.top_layer - 1 {
                        b.spawn(Text2dBundle {
                            text: Text::from_section(
                                format!("{},{},{}", tile.x, tile.y, tile.z),
                                TextStyle {
                                    font_size: 10.0,
                                    color: Color::BLACK,
                                    ..default()
                                },
                            ),
                            transform: Transform::from_xyz(0.0, 16.0, 10.0),
                            ..default()
                        });
                    }
                })
                .id();
            if i == 15 {
                commands.spawn((
                    SpriteBundle {
                        texture: asset_server.load("sprites/dummy.png"),
                        transform: Transform::from_xyz(pos.x, pos.y, pos.z + 1.),
                        ..default()
                    },
                    Mob {},
                    Position::new(tile.x, tile.y, tile.z),
                    PathfindingSteps {
                        steps: VecDeque::new(),
                    },
                ));
            }
            (tile, entity)
        })
        .collect();
    commands.insert_resource(IsoGrid {
        entities,
        layout,
        blocked_coords,
    });

    // TODO this should be moved out
    //map_creation_state.set(WorldCreationState::Disabled);
    //app_state.set(GameState::InGame);
}
/// Input interaction
fn handle_input(
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut mob_query: Query<(&Position, &mut PathfindingSteps), With<Mob>>,
    mut current: Local<Position>,
    mut grid: ResMut<IsoGrid>,
    mut tiles: Query<&mut TextureAtlas>,
) {
    // Get the cursor position
    let window = windows.single();
    let (camera, cam_transform) = cameras.single();
    if let Some(pos) = window
        .cursor_position()
        .and_then(|p| camera.viewport_to_world_2d(cam_transform, p))
    {
        // Translate the cursor position to tile position
        let all_tile_pos_v = grid.layout.world_pos_to_tile(pos);
        // Check if there is a tile where the cursor is. If there are several of them pick the top one
        let tile_pos_v: Vec<Position> = all_tile_pos_v
            .into_iter()
            .filter(|tile| grid.entities.get(tile).copied().is_some())
            .collect();
        let Some(tile_pos) = tile_pos_v.last() else {
            return;
        };
        let Some(entity) = grid.entities.get(tile_pos).copied() else {
            return;
        };

        // Send the mob to the left clicked position
        if buttons.just_pressed(MouseButton::Left) {
            let (mob_pos, mut mob_steps) = mob_query.single_mut();
            if *tile_pos == *current {
                return;
            }
            *current = *tile_pos;
            let Some(path) = a_star(*mob_pos, *tile_pos, |_, h| {
                (grid.entities.contains_key(&h) && !grid.blocked_coords.contains(&h)).then_some(1)
            }) else {
                log::info!("No path found");
                return;
            };
            mob_steps.steps.clear();
            mob_steps.steps = path
                .into_iter()
                .inspect(|h| {
                    if grid.blocked_coords.contains(h) {
                        log::error!("A star picked a blocked coord: {h:?}");
                    }
                })
                .collect();
        }

        // TODO this conflicts with the highlight tiles
        // Put wall block on right click
        if buttons.just_pressed(MouseButton::Right) {
            if grid.blocked_coords.contains(tile_pos) {
                grid.blocked_coords.remove(tile_pos);
                let Ok(mut atlas) = tiles.get_mut(entity) else {
                    return;
                };
                atlas.index = 5;
            } else {
                grid.blocked_coords.insert(*tile_pos);
                let Ok(mut atlas) = tiles.get_mut(entity) else {
                    return;
                };
                atlas.index = 2;
            }
        }
    }
}

fn hover_highlight(
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    grid: ResMut<IsoGrid>,
    mut tiles: Query<&mut TextureAtlas>,
    mut selected_tile: ResMut<SelectedTile>,
) {
    // Get the cursor position
    let window = windows.single();
    let (camera, cam_transform) = cameras.single();
    if let Some(pos) = window
        .cursor_position()
        .and_then(|p| camera.viewport_to_world_2d(cam_transform, p))
    {
        // Translate the cursor position to tile position
        let all_tile_pos_v = grid.layout.world_pos_to_tile(pos);
        // Check if there is a tile where the cursor is. If there are several of them pick the top one
        let tile_pos_v: Vec<Position> = all_tile_pos_v
            .into_iter()
            .filter(|tile| grid.entities.get(tile).copied().is_some())
            .collect();
        let Some(tile_pos) = tile_pos_v.last() else {
            return;
        };
        let Some(entity) = grid.entities.get(tile_pos).copied() else {
            return;
        };
        // Highlight hovered tile
        // Clean previous selected tile
        if let Some(selected_entity) = selected_tile.selected {
            if *tile_pos != selected_entity {
                if let Ok(mut atlas2) = tiles.get_mut(grid.entities[&selected_entity]) {
                    atlas2.index -= 1;
                    // Retrieve the texture for the tile
                    let Ok(mut atlas) = tiles.get_mut(entity) else {
                        return;
                    };
                    // Mark the tile as selected
                    selected_tile.selected = Some(*tile_pos);
                    atlas.index += 1;
                }
            }
        } else {
            // Retrieve the texture for the tile
            let Ok(mut atlas) = tiles.get_mut(entity) else {
                return;
            };
            // Mark the tile as selected
            selected_tile.selected = Some(*tile_pos);
            atlas.index += 1;
        }
    }
}

fn move_mob(
    time: Res<Time>,
    mut move_timer: ResMut<MoveTimer>,
    mut mob_query: Query<(&mut Transform, &mut Position, &mut PathfindingSteps), With<Mob>>,
    grid: ResMut<IsoGrid>,
) {
    move_timer.0.tick(time.delta());
    if move_timer.0.tick(time.delta()).just_finished() {
        // this should be multiple mobs in the future
        // this ideally should calculate the direction to go between the actual position with the
        // next step and then move the mob in that direction
        let (mut mob_transform, mut mob_pos, mut mob_step) = mob_query.single_mut();
        let Some(next_step) = mob_step.steps.pop_front() else {
            return;
        };
        // TODO check after updating the blocked_coords?
        if grid.blocked_coords.contains(&next_step) {
            mob_step.steps.clear();
            return;
        }

        let step = grid.layout.tile_to_world_pos(next_step);

        mob_transform.translation.x = step.x;
        mob_transform.translation.y = step.y;
        mob_transform.translation.z = step.z + 1.0;

        *mob_pos = next_step;
    }
}
