use super::{matrix::ProjectionMatrix, CHUNK_DIMENSIONS};
use crate::Position;
use bevy::prelude::{Vec2, Vec3};

const TILE_SIZE: Vec2 = Vec2::splat(32.0);

#[derive(Debug, Clone)]
pub struct Layout {
    pub projection: ProjectionMatrix,
    pub origin: Vec3,
    pub tile_size: Vec2,
    pub top_layer: i32,
}

impl Layout {
    #[must_use]
    #[inline]
    /// Computes isometric coordinates `iso` into world/pixel coordinates
    pub fn tile_to_world_pos(&self, pos: Position) -> Vec3 {
        let [x, y, z] = self.projection.forward(pos.to_array_f32());
        Vec3::new(x * self.tile_size.x, y * self.tile_size.y, z) + self.origin
    }
}

impl Default for Layout {
    fn default() -> Self {
        let offset_layers = TILE_SIZE.y / 2.0 * 3.0;
        let offset_center_tile = TILE_SIZE.y / 4.0;
        Self {
            projection: ProjectionMatrix::default(),
            origin: Vec3::new(0., -(offset_layers + offset_center_tile), 0.),
            tile_size: TILE_SIZE,
            top_layer: CHUNK_DIMENSIONS.2,
        }
    }
}
