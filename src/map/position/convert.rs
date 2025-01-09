use crate::Position;
use bevy::prelude::{Vec2, Vec3};

impl From<(i32, i32, i32)> for Position {
    #[inline]
    fn from((x, y, z): (i32, i32, i32)) -> Self {
        Self::new(x, y, z)
    }
}

impl From<[f32; 3]> for Position {
    #[inline]
    fn from(v: [f32; 3]) -> Self {
        Self::round(v)
    }
}

impl From<Vec3> for Position {
    #[inline]
    fn from(value: Vec3) -> Self {
        Self::from(value.to_array())
    }
}

// TODO delete this and fix its dependencies
impl From<Vec2> for Position {
    #[inline]
    fn from(value: Vec2) -> Self {
        Self::from(Vec3::new(value.x, value.y, 0.0))
    }
}
