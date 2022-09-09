use bevy::{
    math::{UVec2, Vec2},
    prelude::{BuildChildren, Color, Commands, Entity, Transform},
};

use super::{
    map::{TilemapId, TilemapSize, TilemapTileSize},
    tiles::{TileBundle, TileColor, TilePos, TileStorage, TileTexture},
    TilemapGridSize,
};

/// Converts a tile position into an index in a vector.
pub fn pos_2d_to_index(tile_pos: &TilePos, size: &TilemapSize) -> usize {
    ((tile_pos.y * size.x as u32) + tile_pos.x) as usize
}

/// Calculates the position of the bottom-left of a chunk with the specified position.
///
/// This calculation is mostly used internally for rendering but it might be helpful so it's exposed here.
pub fn get_chunk_2d_transform(
    chunk_position: UVec2,
    chunk_size: UVec2,
    z_index: u32,
    grid_size: Vec2,
) -> Transform {
    // Get the position of the bottom left tile of the chunk: the "anchor tile".
    let anchor_tile_pos = TilePos {
        x: chunk_position.x * chunk_size.x,
        y: chunk_position.y * chunk_size.y,
    };
    let grid_size: TilemapGridSize = grid_size.into();
    // Now get the position of the anchor tile.
    let r = get_tile_pos_in_world_space(&anchor_tile_pos, &grid_size);
    Transform::from_xyz(r.x, r.y, z_index as f32)
}

/// Returns the bottom-left coordinate of the tile associated with the specified `tile_pos`.
pub fn get_tile_pos_in_world_space(
    tile_pos: &TilePos,
    grid_size: &TilemapGridSize,
) -> Vec2 {
    let tile_pos_f32: Vec2 = tile_pos.into();
    let grid_size: Vec2 = grid_size.into();
    let mut pos = Vec2::new(grid_size.x * tile_pos_f32.x, grid_size.y * tile_pos_f32.y);

    pos = project(tile_pos_f32.x, tile_pos_f32.y, grid_size.x, grid_size.y);
    pos
}

/// Fills an entire tile storage with the given tile.
pub fn fill_tilemap(
    tile_texture: TileTexture,
    size: TilemapSize,
    tilemap_id: TilemapId,
    commands: &mut Commands,
    tile_storage: &mut TileStorage,
) {
    for x in 0..size.x {
        for y in 0..size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn()
                .insert_bundle(TileBundle {
                    position: tile_pos,
                    tilemap_id,
                    texture: tile_texture,
                    ..Default::default()
                })
                .id();
            commands.entity(tilemap_id.0).add_child(tile_entity);
            tile_storage.set(&tile_pos, Some(tile_entity));
        }
    }
}

/// Fills a rectangular region with the given tile.
///
/// The rectangular region is defined by an `origin` in `TilePos`, and a size
/// in tiles (`TilemapSize`).  
pub fn fill_tilemap_rect(
    tile_texture: TileTexture,
    origin: TilePos,
    size: TilemapSize,
    tilemap_id: TilemapId,
    commands: &mut Commands,
    tile_storage: &mut TileStorage,
) {
    for x in 0..size.x {
        for y in 0..size.y {
            let tile_pos = TilePos {
                x: origin.x + x,
                y: origin.y + y,
            };

            let tile_entity = commands
                .spawn()
                .insert_bundle(TileBundle {
                    position: tile_pos,
                    tilemap_id,
                    texture: tile_texture,
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, Some(tile_entity));
        }
    }
}

/// Fills a rectangular region with colored versions of the given tile.
///
/// The rectangular region is defined by an `origin` in `TilePos`, and a size
/// in tiles (`TilemapSize`).  
pub fn fill_tilemap_rect_color(
    tile_texture: TileTexture,
    origin: TilePos,
    size: TilemapSize,
    color: Color,
    tilemap_id: TilemapId,
    commands: &mut Commands,
    tile_storage: &mut TileStorage,
) {
    for x in 0..size.x {
        for y in 0..size.y {
            let tile_pos = TilePos {
                x: origin.x + x,
                y: origin.y + y,
            };

            let tile_entity = commands
                .spawn()
                .insert_bundle(TileBundle {
                    position: tile_pos,
                    tilemap_id,
                    texture: tile_texture,
                    color: TileColor(color),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, Some(tile_entity));
        }
    }
}

/// Calculates a tilemap's centered position.
pub fn get_centered_transform_2d(
    size: &TilemapSize,
    tile_size: &TilemapTileSize,
    z_index: f32,
) -> Transform {
    Transform::from_xyz(
        -(size.x as f32 * tile_size.x as f32) / 2.0,
        -(size.y as f32 * tile_size.y as f32) / 2.0,
        z_index,
    )
}

/// Projects a 2D screen space point into isometric diamond space.
///
/// `grid_width` and `grid_height` are the dimensions of the grid in pixels.
pub fn project(x: f32, y: f32, grid_width: f32, grid_height: f32) -> Vec2 {
    let dx = grid_width / 2.0;
    let dy = grid_height / 2.0;

    let new_x = (x + y) * dx;
    let new_y = (-x + y) * dy;
    Vec2::new(new_x, new_y)
}

#[derive(Clone, Copy, Debug)]
pub enum NeighborDirection {
    North,
    NorthWest,
    West,
    SouthWest,
    South,
    SouthEast,
    East,
    NorthEast,
}

impl NeighborDirection {
    fn next_direction(&self) -> NeighborDirection {
        use NeighborDirection::*;
        match self {
            North => NorthWest,
            NorthWest => West,
            West => SouthWest,
            SouthWest => South,
            South => SouthEast,
            SouthEast => East,
            East => NorthEast,
            NorthEast => North,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Neighbors<T: Copy> {
    pub north: Option<T>,
    pub north_west: Option<T>,
    pub west: Option<T>,
    pub south_west: Option<T>,
    pub south: Option<T>,
    pub south_east: Option<T>,
    pub east: Option<T>,
    pub north_east: Option<T>,
}

pub struct NeighborsIntoIterator<T: Copy> {
    neighbors: Neighbors<T>,
    /// The next direction the iterator will output.
    cursor: Option<NeighborDirection>,
}

impl<T: Copy> NeighborsIntoIterator<T> {
    fn get_at_cursor(&self) -> Option<T> {
        self.cursor.and_then(|direction| match direction {
            NeighborDirection::North => self.neighbors.north,
            NeighborDirection::NorthWest => self.neighbors.north_west,
            NeighborDirection::West => self.neighbors.west,
            NeighborDirection::SouthWest => self.neighbors.south_west,
            NeighborDirection::South => self.neighbors.south,
            NeighborDirection::SouthEast => self.neighbors.south_east,
            NeighborDirection::East => self.neighbors.east,
            NeighborDirection::NorthEast => self.neighbors.north_east,
        })
    }
}

impl<T: Copy> Iterator for NeighborsIntoIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.cursor.and_then(|direction| {
            let neighbor = self.get_at_cursor();
            match direction {
                NeighborDirection::NorthEast => {
                    self.cursor = None;
                    neighbor
                }
                direction => {
                    self.cursor = Some(direction.next_direction());
                    neighbor.or_else(|| self.next())
                }
            }
        })
    }
}

impl<T: Copy> IntoIterator for Neighbors<T> {
    type Item = T;
    type IntoIter = NeighborsIntoIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        NeighborsIntoIterator {
            neighbors: self,
            cursor: Some(NeighborDirection::North),
        }
    }
}

impl<T: Copy> Neighbors<T> {
    pub fn count(&self) -> usize {
        self.into_iter().map(|_| 1).sum()
    }
}

impl Neighbors<Entity> {
    fn from_neighboring_pos(
        neighbors: &Neighbors<TilePos>,
        tile_storage: &TileStorage,
    ) -> Neighbors<Entity> {
        Neighbors {
            north: neighbors.north.and_then(|pos| tile_storage.get(&pos)),
            south: neighbors.south.and_then(|pos| tile_storage.get(&pos)),
            east: neighbors.east.and_then(|pos| tile_storage.get(&pos)),
            west: neighbors.west.and_then(|pos| tile_storage.get(&pos)),
            north_east: neighbors.north_east.and_then(|pos| tile_storage.get(&pos)),
            north_west: neighbors.north_west.and_then(|pos| tile_storage.get(&pos)),
            south_east: neighbors.south_east.and_then(|pos| tile_storage.get(&pos)),
            south_west: neighbors.south_west.and_then(|pos| tile_storage.get(&pos)),
        }
    }
}

/// Retrieves a list of neighbors for the given tile position.
///
/// If a particular neighboring position does not exist in the provided storage,
/// then it will not be returned.
pub fn get_tile_neighbors(
    tile_pos: &TilePos,
    tile_storage: &TileStorage,
) -> Neighbors<Entity> {
    Neighbors::from_neighboring_pos(
        &get_neighboring_pos(tile_pos, &tile_storage.size),
        tile_storage,
    )
}

/// Retrieves the positions of neighbors of the tile with the specified position.
///
/// Tile positions are bounded:
///     * between `0` and `tilemap_size.x` in the `x` position,
///     * between `0` and `tilemap_size.y` in the `y` position.
/// Directions in the returned [`Neighbor`](crate::helpers::Neighbor) struct with tile coordinates that violate these requirements will be set to `None`.
pub fn get_neighboring_pos(
    tile_pos: &TilePos,
    tilemap_size: &TilemapSize,
) -> Neighbors<TilePos> {
    neighbor_pos_with_diagonals(tile_pos, tilemap_size)
}

impl TilePos {
    #[inline]
    fn plus_x(&self, tilemap_size: &TilemapSize) -> Option<TilePos> {
        if self.x < tilemap_size.x - 1 {
            Some(TilePos {
                x: self.x + 1,
                y: self.y,
            })
        } else {
            None
        }
    }

    #[inline]
    fn plus_y(&self, tilemap_size: &TilemapSize) -> Option<TilePos> {
        if self.y < tilemap_size.y - 1 {
            Some(TilePos {
                x: self.x,
                y: self.y + 1,
            })
        } else {
            None
        }
    }

    #[inline]
    fn plus_xy(&self, tilemap_size: &TilemapSize) -> Option<TilePos> {
        if self.x < tilemap_size.x - 1 && self.y < tilemap_size.y - 1 {
            Some(TilePos {
                x: self.x + 1,
                y: self.y + 1,
            })
        } else {
            None
        }
    }

    #[inline]
    fn minus_x(&self) -> Option<TilePos> {
        if self.x != 0 {
            Some(TilePos {
                x: self.x - 1,
                y: self.y,
            })
        } else {
            None
        }
    }

    #[inline]
    fn minus_y(&self) -> Option<TilePos> {
        if self.y != 0 {
            Some(TilePos {
                x: self.x,
                y: self.y - 1,
            })
        } else {
            None
        }
    }

    #[inline]
    fn minus_xy(&self) -> Option<TilePos> {
        if self.x != 0 && self.y != 0 {
            Some(TilePos {
                x: self.x - 1,
                y: self.y - 1,
            })
        } else {
            None
        }
    }

    #[inline]
    fn plus_x_minus_y(&self, tilemap_size: &TilemapSize) -> Option<TilePos> {
        if self.x < tilemap_size.x - 1 && self.y != 0 {
            Some(TilePos {
                x: self.x + 1,
                y: self.y - 1,
            })
        } else {
            None
        }
    }

    #[inline]
    fn plus_x_minus_2y(&self, tilemap_size: &TilemapSize) -> Option<TilePos> {
        if self.x < tilemap_size.x - 1 && self.y > 1 {
            Some(TilePos {
                x: self.x + 1,
                y: self.y - 2,
            })
        } else {
            None
        }
    }

    #[inline]
    fn minus_x_plus_y(&self, tilemap_size: &TilemapSize) -> Option<TilePos> {
        if self.y < tilemap_size.y - 1 && self.x != 0 {
            Some(TilePos {
                x: self.x - 1,
                y: self.y + 1,
            })
        } else {
            None
        }
    }

    #[inline]
    fn minus_x_plus_2y(&self, tilemap_size: &TilemapSize) -> Option<TilePos> {
        if self.y < tilemap_size.y - 2 && self.x != 0 {
            Some(TilePos {
                x: self.x - 1,
                y: self.y + 2,
            })
        } else {
            None
        }
    }

    fn north(&self, tilemap_size: &TilemapSize) -> Option<TilePos> {
        self.plus_y(tilemap_size)
    }

    fn north_west(&self, tilemap_size: &TilemapSize) -> Option<TilePos> {
        self.minus_x_plus_y(tilemap_size)
    }

    fn west(&self) -> Option<TilePos> {
        self.minus_x()
    }

    fn south_west(&self) -> Option<TilePos> {
        self.minus_xy()
    }

    fn south(&self) -> Option<TilePos> {
        self.minus_y()
    }

    fn south_east(&self, tilemap_size: &TilemapSize) -> Option<TilePos> {
        self.plus_x_minus_y(tilemap_size)
    }

    fn east(&self, tilemap_size: &TilemapSize) -> Option<TilePos> {
        self.plus_x(tilemap_size)
    }

    fn north_east(&self, tilemap_size: &TilemapSize) -> Option<TilePos> {
        self.plus_xy(tilemap_size)
    }
}

/// Retrieves the positions of neighbors of the tile with the specified position, assuming
/// that 1) the tile exists on [`Square`](crate::map::TilemapType::Square) tilemap
/// and 2) neighbors **do** include tiles located diagonally across from the specified position.
///
/// Tile positions are bounded:
///     * between `0` and `tilemap_size.x` in the `x` position,
///     * between `0` and `tilemap_size.y` in the `y` position.
/// Directions in the returned [`Neighbor`](crate::helpers::Neighbor) struct with tile coordinates that violate these requirements will be set to `None`.
pub fn neighbor_pos_with_diagonals(
    tile_pos: &TilePos,
    tilemap_size: &TilemapSize,
) -> Neighbors<TilePos> {
    Neighbors {
        north: tile_pos.north(tilemap_size),
        north_west: tile_pos.north_west(tilemap_size),
        west: tile_pos.west(),
        south_west: tile_pos.south_west(),
        south: tile_pos.south(),
        south_east: tile_pos.south_east(tilemap_size),
        east: tile_pos.east(tilemap_size),
        north_east: tile_pos.north_east(tilemap_size),
    }
}
