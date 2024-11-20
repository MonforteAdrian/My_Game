use super::{matrix::ProjectionMatrix, Position, CHUNK_DIMENSIONS};
use bevy::prelude::{Vec2, Vec3};

// This should be resources?
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

    #[must_use]
    #[inline]
    #[allow(clippy::cast_precision_loss)]
    /// Computes world/pixel coordinates `pos` into isometric coordinates
    pub fn world_pos_to_tile(&self, pos: Vec2) -> Vec<Position> {
        (0..self.top_layer)
            .map(move |layer| {
                let point = Vec3::new(
                    (pos.x - self.origin.x) / self.tile_size.x,
                    (pos.y - self.origin.y - self.tile_size.y / 4.) / self.tile_size.y,
                    layer as f32,
                );
                let [x, y, z] = self.projection.inverse(point.to_array());
                let p = Vec3::new(x, y, z);
                Position::round(p.to_array())
            })
            .collect::<Vec<Position>>()
    }
}

impl Default for Layout {
    fn default() -> Self {
        Self {
            projection: ProjectionMatrix::default(),
            origin: Vec3::ZERO,
            tile_size: Vec2::ONE,
            top_layer: CHUNK_DIMENSIONS.2,
        }
    }
}
