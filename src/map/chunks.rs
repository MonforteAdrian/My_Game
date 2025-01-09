use super::Layer;
use crate::Position;
use bevy::prelude::Component;
use std::cmp::Reverse;

/// (columns, rows, layers)
pub const CHUNK_DIMENSIONS: (i32, i32, i32) = (12, 12, 1);

/// Chunk parameters.
#[derive(Component, Debug, Clone)]
pub struct Chunk {
    /// x coordinate of the chunk
    pub x: i32,
    /// y coordinate of the chunk
    pub y: i32,
    /// Iterator of the layers of the 'Chunk'
    pub layers: Vec<Layer>,
}

impl Chunk {
    /// Generates a new chunk with given x and y coordinates
    #[must_use]
    pub fn new(x: i32, y: i32) -> Self {
        let layers =
            (0..CHUNK_DIMENSIONS.2).map(|z| Layer::new(x, y, z, CHUNK_DIMENSIONS.0, CHUNK_DIMENSIONS.1)).collect();
        Self { x, y, layers }
    }
}

/// Generates the number of chunks needed to fill the window
#[must_use]
pub fn generate_mesh_of_chunks(max_x: i32, min_x: i32, max_y: i32, min_y: i32) -> Vec<Chunk> {
    (min_y..=max_y).rev().flat_map(move |y| (min_x..=max_x).map(move |x| Chunk::new(x, y))).collect()
}

/// Function to collect all tiles from chunks and sort them by z, then y, then x
#[must_use]
pub fn get_sorted_tiles(chunks: Vec<Chunk>) -> Vec<Position> {
    let mut tiles: Vec<Position> =
        chunks.into_iter().flat_map(|chunk| chunk.layers.into_iter().flat_map(|layer| layer.tiles)).collect();

    tiles.sort_by_key(|tile| (tile.z, Reverse(tile.y), Reverse(tile.x)));
    tiles
}

// TODO create tests
#[cfg(test)]
mod tests {
    use super::*;
}
