pub mod components;
pub mod resources;

use bevy::log;
use bevy::prelude::*;
use bevy::math::Vec3Swizzles;
use bounds::Bounds2;
use components::Coordinates;
use resources::tile::Tile;
use resources::tile_map::TileMap;
use resources::Board;
use resources::TileSize;
use resources::BoardOptions;
use resources::BoardPosition;
#[cfg(feature = "debug")]
use bevy_inspector_egui::RegisterInspectable;
use components::*;

mod bounds;
mod systems;

pub struct MyPlugin;

impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::create_board)
            .add_system(systems::input::input_handling);
        log::info!("Loaded Board Plugin");
        #[cfg(feature = "debug")]
        {
            app.register_inspectable::<Coordinates>();
            app.register_inspectable::<Grass>();
            app.register_inspectable::<Dirt>();
            app.register_inspectable::<Stone>();
        }
    }
}

impl MyPlugin {
    pub fn create_board(
        mut commands: Commands,
        board_options: Option<Res<BoardOptions>>,
        window: Res<WindowDescriptor>,
        asset_server: Res<AssetServer>,
    ) {
        let options = match board_options {
            None => BoardOptions::default(),
            Some(o) => o.clone(),
        };

        //let font = asset_server.load("fonts/pixeled.ttf");
        let cube_image = asset_server.load("sprites/cube.png");

        let mut tile_map = TileMap::empty(options.map_size.0, options.map_size.1);
        #[cfg(feature = "debug")]
        log::info!("{}", tile_map.console_output());

        let tile_size = match options.tile_size {
            TileSize::Fixed(v) => v,
            TileSize::Adaptive { min, max } => Self::adaptative_tile_size(
                window,
                (min, max),
                (tile_map.width(), tile_map.height()),
            ),
        };

        let board_size = Vec2::new(
            tile_map.width() as f32 * tile_size,
            tile_map.height() as f32 * tile_size,
        );
        log::info!("board size: {}", board_size);

        let board_position = match options.position {
            BoardPosition::Centered { offset } => {
                Vec3::new(-(board_size.x / 2.), -(board_size.y / 2.), 0.)
            }
            BoardPosition::Custom(p) => p,
        };

        commands
            .spawn()
            .insert(Name::new("Board"))
            .insert(Transform::from_translation(board_position))
            .insert(GlobalTransform::default())
            .with_children(|parent| {
                Self::spawn_tiles(
                    parent,
                    &tile_map,
                    tile_size,
                    options.tile_padding,
                    cube_image,
                );
            });
        commands.insert_resource(Board {
            tile_map,
            bounds: Bounds2 {
                position: board_position.xy(),
                size: board_size,
            },
            tile_size,
        });
    }

    fn spawn_tiles(
            parent: &mut ChildBuilder,
            tile_map: &TileMap,
            size: f32,
            padding: f32,
            cube_image: Handle<Image>,
        ) {
            for (y, line) in tile_map.iter().enumerate() {
                for (x, tile) in line.iter().enumerate() {
                    let coordinates = Coordinates {
                        x: x as u16,
                        y: y as u16,
                    };
                    let mut cmd = parent.spawn();
                    cmd.insert_bundle(SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::splat(size - padding)),
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(
                            (x as f32 * size) + (size / 2.),
                            (y as f32 * size) + (size / 2.),
                            1.,
                        ),
                        ..Default::default()
                    })
                    .insert(Name::new(format!("Tile ({}, {})", x, y)))
                    .insert(coordinates);

                    match tile {
                        Tile::Block => {
                            cmd.insert(Grass);
                            cmd.with_children(|parent| {
                                parent.spawn_bundle(SpriteBundle {
                                    sprite: Sprite {
                                        custom_size: Some(Vec2::splat(size - padding)),
                                        ..Default::default()
                                    },
                                    transform: Transform::from_xyz(0., 0., 1.),
                                    texture: cube_image.clone(),
                                    ..Default::default()
                                });
                            });
                        }
                        Tile::Empty => (),
                    }
                }
            }
    }

    fn adaptative_tile_size(
        window: Res<WindowDescriptor>,
        (min, max): (f32, f32),
        (width, height): (u16, u16),
    ) -> f32 {
        let max_width = window.width / width as f32;
        let max_height = window.height / height as f32;
        max_width.min(max_height).clamp(min, max)
    }
}
