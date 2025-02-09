use super::CHUNK_DIMENSIONS;
use crate::{Position, TileType};
use bevy::prelude::Component;

#[derive(PartialEq, Clone, Debug)]
pub struct TileData {
    pub pos: Position,
    pub tile_type: TileType,
}

/// A layer of a chunk, containing tiles and its z-coordinate.
#[derive(Component, Debug, Clone)]
pub struct Layer {
    /// The z-coordinate of this layer.
    pub z: i32,
    /// The tiles (positions) contained in this layer.
    pub tiles: Vec<TileData>,
}

impl Layer {
    /// Creates a new `Layer` for the given chunk coordinates and `z` level.
    ///
    /// # Parameters
    /// - `chunk_x`: The x-coordinate of the chunk.
    /// - `chunk_y`: The y-coordinate of the chunk.
    /// - `z`: The z-coordinate of the layer.
    ///
    /// # Returns
    /// A new `Layer` instance with tiles adjusted for the chunk position.
    #[must_use]
    pub fn new(chunk_x: i32, chunk_y: i32, z: i32) -> Self {
        let mut layer = Self::default();
        layer.z = z;
        layer.tiles.iter_mut().for_each(|tile| {
            let _ = tile.pos.to_absolute((chunk_x, chunk_y));
            tile.pos.z = z
        });
        layer
    }
}

impl Default for Layer {
    fn default() -> Self {
        let z = 0;
        let chunk_side = CHUNK_DIMENSIONS.0;
        // Base offsets for rows and columns
        let base_x_offset = -(chunk_side / 4);
        let base_y_offset = chunk_side - chunk_side / 3;

        let tiles = (0..chunk_side)
            .flat_map(|row| {
                // Precompute row-based offsets
                let row_x_offset = base_x_offset - (row / 2);
                let row_y_offset = base_y_offset - ((1 + row) / 2);

                (0..chunk_side).map(move |col| TileData {
                    pos: Position::new(row_x_offset + col, row_y_offset - col, z),
                    tile_type: TileType::Block,
                })
            })
            .collect();
        Self { z, tiles }
    }
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//    const CHUNK_DIMENSIONS: (i32, i32) = (16, 16); // Example
//
//    #[test]
//    fn test_default_layer() {
//        let layer = Layer::default();
//
//        assert_eq!(layer.z, 0);
//        assert_eq!(
//            layer.tiles.len(),
//            CHUNK_DIMENSIONS.0 as usize * CHUNK_DIMENSIONS.0 as usize
//        );
//    }
//
//    #[test]
//    fn test_layer_new() {
//        let layer = Layer::new(1, 1, 5);
//
//        assert_eq!(layer.z, 5);
//        assert_eq!(
//            layer.tiles.len(),
//            CHUNK_DIMENSIONS.0 as usize * CHUNK_DIMENSIONS.0 as usize
//        );
//
//        // Verify the first and last tiles are adjusted for chunk offsets
//        let first_tile = &layer.tiles[0];
//        assert_eq!(first_tile.z, 5);
//
//        let last_tile = layer.tiles.last().unwrap();
//        assert_eq!(last_tile.z, 5);
//    }
//}
