use super::Layer;
use crate::{Position, TileData, TileType};
use bevy::prelude::Component;
use noise::utils::NoiseMap;
use std::collections::HashMap;

/// (columns, rows, layers)
pub const CHUNK_DIMENSIONS: (i32, i32, i32) = (16, 16, 4);

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
        let layers = (0..CHUNK_DIMENSIONS.2).map(|z| Layer::new(x, y, z)).collect();
        Self { x, y, layers }
    }

    /// Removes the given position from a chunk
    pub fn remove_tile(&mut self, pos: Position) {
        if let Some(layer) = self.layers.get_mut(pos.z as usize) {
            if let Some(index) = layer.tiles.iter().position(|tile| tile.pos == pos) {
                layer.tiles.remove(index);
            }
        }
    }

    // Finds the heighest tile in a given [`Position`] ignoring z
    pub fn find_top_layer(&self, pos: &Position) -> i32 {
        for z in (0..CHUNK_DIMENSIONS.2).rev() {
            if let Some(layer) = self.layers.get(z as usize) {
                if layer
                    .tiles
                    .iter()
                    .any(|tile| tile.pos.x == pos.x && tile.pos.y == pos.y)
                {
                    return z;
                }
            }
        }
        0
    }
}

/// Takes a NoiseMap and map it to the chunks in the position marked by the bounds of the map
pub fn split_map(map: &NoiseMap) -> HashMap<(i32, i32), Chunk> {
    let (map_width, map_height) = (map.size().0 as i32, map.size().1 as i32);
    let (chunk_width, chunk_height, chunk_layers) = CHUNK_DIMENSIONS;

    // Calculate the number of chunks needed based on the map dimensions
    let cols = (map_width / chunk_width) - 1;
    let rows = (map_height / chunk_height) - 1;

    // Create chunks
    let mut chunks = generate_mesh_of_chunks(cols, 0, rows, 0);
    let chunks_side = chunks.len().isqrt();

    for ((x, y), chunk) in chunks.iter_mut() {
        let chunk_copy = chunk.clone();
        for (j, tile) in chunk_copy.layers[0]
            .tiles
            .chunks_exact(chunk_width as usize)
            .rev()
            .flatten()
            .enumerate()
        {
            let noise_x = x * chunk_width + (j as i32 % chunk_width);
            let noise_y = y * chunk_height + (j as i32 / chunk_width);
            if noise_x > map_width || noise_y > map_height || noise_x < 0 || noise_y < 0 {
                break;
            }

            let z = ((map.get_value(noise_x as usize, noise_y as usize) * 0.5 + 0.5).clamp(0.0, 1.0)
                * chunk_layers as f64) as i32;

            // Find the tile at (tile.pos.x, tile.pos.y, z) and change it to Floor
            if let Some(layer) = chunk.layers.get_mut(z as usize) {
                if let Some(tile_at_z) = layer
                    .tiles
                    .iter_mut()
                    .find(|t| t.pos.x == tile.pos.x && t.pos.y == tile.pos.y)
                {
                    tile_at_z.tile_type = TileType::Floor;
                }
            }

            for layer in (z + 1)..chunk_layers {
                chunk.remove_tile(Position {
                    x: tile.pos.x,
                    y: tile.pos.y,
                    z: layer,
                });
            }
        }
    }

    chunks
}

/// Generates the number of chunks needed to fill the window

#[must_use]
pub fn generate_mesh_of_chunks(max_x: i32, min_x: i32, max_y: i32, min_y: i32) -> HashMap<(i32, i32), Chunk> {
    (min_y..=max_y)
        .rev()
        .flat_map(move |y| {
            (min_x..=max_x).map(move |x| {
                let chunk = Chunk::new(x, y);
                ((x, y), chunk)
            })
        })
        .collect()
}
/// Function to collect all tiles from chunks and sort them by z, then y, then x
#[must_use]
pub fn get_sorted_tiles(chunks: &HashMap<(i32, i32), Chunk>) -> Vec<TileData> {
    let mut tiles: Vec<TileData> = chunks
        .values()
        .flat_map(|chunk| chunk.layers.iter().flat_map(|layer| layer.tiles.iter().cloned()))
        .collect();

    tiles.sort_by_key(|tile| (tile.pos.z, std::cmp::Reverse(tile.pos.y), std::cmp::Reverse(tile.pos.x)));
    tiles
}

// TODO create tests
#[cfg(test)]
mod tests {
    use super::*;
}
