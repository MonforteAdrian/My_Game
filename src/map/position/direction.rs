use bevy::prelude::{Component, Reflect};

/// Default is 0 that correspond to the +X axis East and where Gandalf shall come
#[derive(Component, Reflect, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct Direction(pub(crate) u8);

#[allow(dead_code)]
impl Direction {
    /// Direction to (1, 0, 0)
    /// Direction towards `X`
    pub const X: Self = Self(0);
    ///
    /// Represents "East" neighbor in isometric projection
    pub const EAST: Self = Self(0);

    /// Direction to (1, 1, 0)
    /// Direction towards `X, Y`
    pub const X_Y: Self = Self(1);
    ///
    /// Represents "North East" neighbor in isometric projection
    pub const NORTH_EAST: Self = Self(1);

    /// Direction to (0, 1, 0)
    /// Direction towards `Y`
    pub const Y: Self = Self(2);
    ///
    /// Represents "North" neighbor in isometric projection
    pub const NORTH: Self = Self(2);

    /// Direction to (-1, 1, 0)
    /// Direction towards `-X, Y`
    pub const NEG_X_Y: Self = Self(3);
    ///
    /// Represents "North West" neighbor in isometric projection
    pub const NORTH_WEST: Self = Self(3);

    /// Direction to (-1, 0, 0)
    /// Direction towards `-X`
    pub const NEG_X: Self = Self(4);
    ///
    /// Represents "West" neighbor in isometric projection
    pub const WEST: Self = Self(4);

    /// Direction to (-1, -1, 0)
    /// Direction towards `-X, -Y`
    pub const NEG_X_NEG_Y: Self = Self(5);
    ///
    /// Represents "South West" neighbor in isometric projection
    pub const SOUTH_WEST: Self = Self(5);

    /// Direction to (0, -1, 0)
    /// Direction towards `-Y`
    pub const NEG_Y: Self = Self(6);
    ///
    /// Represents "South" neighbor in isometric projection
    pub const SOUTH: Self = Self(6);

    /// Direction to (1, -1, 0)
    /// Direction towards `X, -Y`
    pub const X_NEG_Y: Self = Self(7);
    ///
    /// Represents "South East" neighbor in isometric projection
    pub const SOUTH_EAST: Self = Self(7);

    /// Positionmetric direction coordinates array
    ///
    /// ```txt
    ///          /\
    ///         /1 \
    ///        /\  /\
    ///   +Y  /2 \/0 \ +X
    ///      /\  /\  /\
    ///     /3 \/  \/7 \
    ///     \  /\  /\  /
    ///      \/4 \/6 \/
    ///   -X  \  /\  / -Y
    ///        \/5 \/
    ///         \  /
    ///          \/
    /// ```
    pub const ALL_DIRECTIONS: [Self; 8] = [Self(0), Self(1), Self(2), Self(3), Self(4), Self(5), Self(6), Self(7)];

    #[inline]
    #[must_use]
    /// Converts a `Direction` into its corresponding angle in radians.
    pub fn angle(self) -> f32 {
        self.0 as f32 * std::f32::consts::FRAC_PI_4
    }
}
