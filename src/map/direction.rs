/// Hexagonal neighbor/edge directions
mod edge_direction;
/// Trait implementations
mod impls;
/// Test module
#[cfg(test)]
mod tests;
/// Hexagonal vertex/diagonal directions
mod vertex_direction;
/// Direction way module
mod way;

pub use edge_direction::EdgeDirection;
pub use vertex_direction::VertexDirection;
pub use way::DirectionWay;

/// Angle constants used for directions
pub mod angles {
    /// Angle in radian between two adjacent directions counter clockwise.
    /// Equivalent to 90 degrees
    pub const DIRECTION_ANGLE_RAD: f32 = std::f32::consts::FRAC_PI_2;
    /// Angle in degrees between two adjacent directions counter clockwise.
    /// Equivalent to π / 2 in radians
    pub const DIRECTION_ANGLE_DEGREES: f32 = 90.0;
}
