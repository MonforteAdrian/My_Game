use super::{
    angles::{DIRECTION_ANGLE_DEGREES, DIRECTION_ANGLE_RAD},
    EdgeDirection,
};
use crate::Position;
use bevy::prelude::Vec2;
use std::fmt::Debug;

/// All 4 possible diagonal/vertex directions in isometric space.
///
/// ```txt
///          /\
///         /0 \
///        /\  /\
///   +Y  /  \/  \ +X
///      /\  /\  /\
///     /3 \/  \/1 \
///     \  /\  /\  /
///      \/  \/  \/
///   -X  \  /\  / -Y
///        \/2 \/
///         \  /
///          \/
/// ```
///
/// ## Operations
///
/// Directions can be:
///  - rotated *clockwise* with:
///     - [`Self::clockwise`] and [`Self::rotate_cw`]
///     - The shift right `>>` operator
///  - rotated *counter clockwise* with:
///     - [`Self::counter_clockwise`] and [`Self::rotate_ccw`]
///     - The shift left `<<` operator
///  - negated using the minus `-` operator
///  - multiplied by an `i32`, returning a [`Position`](crate::Position) vector
///
/// Example:
/// ```rust
/// let direction = VertexDirection::RIGHT;
/// assert_eq!(-direction, VertexDirection::LEFT);
/// assert_eq!(direction >> 1, VertexDirection::BOTTOM);
/// assert_eq!(direction << 1, VertexDirection::TOP);
/// ```
///
/// ## Storage
///
/// Both [`EdgeDirection`] and [`VertexDirection`] store a u8 byte between 0 and
/// 3 as following:
///
/// ```txt
///         v0
///         /\
///     e3 /  \ e0
///    v3 /    \
///       \    / v1
///     e2 \  / e1
///         \/
///         v2
/// ```
///
/// On pointy orientation the Positionagon is shifted by 30 degrees clockwise
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(not(target_arch = "spirv"), derive(Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
#[repr(transparent)]
#[doc(alias = "DiagonalDirection")]
pub struct VertexDirection(pub(crate) u8);

impl VertexDirection {
    /// Direction towards `X, Y`
    /// Direction to (1, 1)
    pub const X_Y: Self = Self(0);
    ///
    /// Represents "Top"
    pub const TOP: Self = Self(0);
    ///
    /// Represents "North"
    pub const NORTH: Self = Self(0);

    /// Direction towards `X, -Y`
    /// Direction to (1, -1)
    pub const X_NEG_Y: Self = Self(1);
    ///
    /// Represents "Right"
    pub const RIGHT: Self = Self(1);
    ///
    /// Represents "East"
    pub const EAST: Self = Self(1);

    /// Direction towards `-X, -Y`
    /// Direction to (-1, -1)
    pub const NEG_X_NEG_Y: Self = Self(2);
    ///
    /// Represents "Bottom"
    pub const BOTTOM: Self = Self(2);
    ///
    /// Represents "South"
    pub const SOUTH: Self = Self(2);

    /// Direction towards `-X, Y`
    /// Direction to (-1, 1)
    pub const NEG_X_Y: Self = Self(3);
    ///
    /// Represents "Left"
    pub const LEFT: Self = Self(3);
    ///
    /// Represents "West"
    pub const WEST: Self = Self(3);

    /// All 4 diagonal directions matching
    /// [`Position::DIAGONAL_COORDS`](crate::Position::DIAGONAL_COORDS)
    ///
    /// ```txt
    ///          /\
    ///         /3 \
    ///        /\  /\
    ///   +Y  /  \/  \ +X
    ///      /\  /\  /\
    ///     /2 \/  \/0 \
    ///     \  /\  /\  /
    ///      \/  \/  \/
    ///   -X  \  /\  / -Y
    ///        \/1 \/
    ///         \  /
    ///          \/
    /// ```
    pub const ALL_DIRECTIONS: [Self; 4] = [Self(0), Self(1), Self(2), Self(3)];

    /// Iterates through all directions in clockwise order
    #[must_use]
    pub fn iter() -> impl ExactSizeIterator<Item = Self> {
        Self::ALL_DIRECTIONS.into_iter()
    }

    /// Returns the inner index of the edge direction, from 0 to 3
    #[must_use]
    #[inline]
    pub const fn index(self) -> u8 {
        self.0
    }

    /// Converts the direction to a Position
    #[must_use]
    #[inline]
    pub const fn into_position(self) -> Position {
        Position::DIAGONAL_COORDS[self.0 as usize]
    }

    /// Computes the opposite direction of `self`
    ///
    /// # Example
    ///
    /// ```rust
    /// assert_eq!(VertexDirection::X_Y.const_neg(), VertexDirection::NEG_X_NEG_Y);
    /// ```
    #[must_use]
    #[inline]
    pub const fn const_neg(self) -> Self {
        Self((self.0 + 2) % 4)
    }

    /// Returns the next direction in clockwise order
    ///
    /// # Example
    ///
    /// ```rust
    /// assert_eq!(VertexDirection::X_Y.clockwise(), VertexDirection::X_NEG_Y);
    /// ```
    #[must_use]
    #[inline]
    #[doc(alias = "cw")]
    pub const fn clockwise(self) -> Self {
        Self((self.0 + 1) % 4)
    }

    /// Returns the next direction in counter clockwise order
    ///
    /// # Example
    ///
    /// ```rust
    /// assert_eq!(
    ///     VertexDirection::X_Y.counter_clockwise(),
    ///     VertexDirection::NEG_X_Y
    /// );
    /// ```
    #[must_use]
    #[inline]
    #[doc(alias = "ccw")]
    pub const fn counter_clockwise(self) -> Self {
        Self((self.0 + 3) % 4)
    }

    #[must_use]
    #[inline]
    /// Rotates `self` counter clockwise by `offset` amount.
    ///
    /// # Example
    ///
    /// ```rust
    /// assert_eq!(VertexDirection::X_Y, VertexDirection::X_Y.rotate_ccw(4));
    /// ```
    pub const fn rotate_ccw(self, offset: u8) -> Self {
        Self((self.0 + 4 - (offset % 4)) % 4)
    }

    #[must_use]
    #[inline]
    /// Rotates `self` clockwise by `offset` amount.
    ///
    /// # Example
    ///
    /// ```rust
    /// assert_eq!(VertexDirection::X_Y, VertexDirection::X_Y.rotate_cw(4));
    /// ```
    pub const fn rotate_cw(self, offset: u8) -> Self {
        Self((self.0 + (offset % 4)) % 4)
    }

    #[must_use]
    #[inline]
    const fn steps_between(self, rhs: Self) -> u8 {
        (self.0 + 4 - rhs.0) % 4
    }

    // TODO the angles thing might be usefull if don't delete
    /// Computes the angle between `a` and `b` in radians.
    #[must_use]
    #[inline]
    pub fn angle_between(a: Self, b: Self) -> f32 {
        a.angle_to(b)
    }

    /// Computes the angle between `a` and `b` in degrees.
    #[must_use]
    #[inline]
    pub fn angle_degrees_between(a: Self, b: Self) -> f32 {
        a.angle_degrees_to(b)
    }

    #[allow(clippy::cast_lossless)]
    #[must_use]
    #[inline]
    /// Computes the angle between `self` and `rhs` in radians.
    pub fn angle_to(self, rhs: Self) -> f32 {
        let steps = self.steps_between(rhs) as f32;
        steps * DIRECTION_ANGLE_RAD
    }

    #[allow(clippy::cast_lossless)]
    #[must_use]
    #[inline]
    /// Computes the angle between `self` and `rhs` in degrees.
    pub fn angle_degrees_to(self, rhs: Self) -> f32 {
        let steps = self.steps_between(rhs) as f32;
        steps * DIRECTION_ANGLE_DEGREES
    }

    #[inline]
    #[must_use]
    /// Returns the angle in radians of the given direction in the given
    /// `orientation`
    pub fn angle(self) -> f32 {
        self.angle_to(Self(0))
    }

    #[inline]
    #[must_use]
    /// Returns the unit vector of the direction in the given `orientation`
    pub fn unit_vector(self) -> Vec2 {
        let angle = self.angle();
        Vec2::new(angle.cos(), angle.sin())
    }

    #[inline]
    #[must_use]
    /// Returns the angle in degrees of the given direction according to its
    /// `orientation`
    ///
    /// See [`Self::angle`] for radians angles
    pub fn angle_degrees(self) -> f32 {
        self.angle_degrees_to(Self(0))
    }

    // TODO fix this example and check this code
    #[must_use]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    /// Returns the direction from the given `angle` in degrees
    ///
    /// # Example
    ///
    /// ```rust
    /// let direction = VertexDirection::from_angle_degrees(15.0);
    /// assert_eq!(direction, VertexDirection::FLAT_BOTTOM_RIGHT);
    /// ```
    pub fn from_angle_degrees(angle: f32) -> Self {
        let angle = angle.rem_euclid(360.0);
        let sector = (angle / DIRECTION_ANGLE_DEGREES).trunc() as u8;
        Self((sector + 1) % 4)
    }

    // TODO fix this example and check this code
    #[must_use]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    /// Returns the direction from the given `angle` in radians
    ///
    /// # Example
    ///
    /// ```rust
    /// let direction = VertexDirection::from_angle(0.26);
    /// assert_eq!(direction, VertexDirection::FLAT_BOTTOM_RIGHT);
    /// ```
    pub fn from_angle(angle: f32) -> Self {
        let angle = angle.rem_euclid(std::f32::consts::TAU);
        let sector = (angle / DIRECTION_ANGLE_RAD) as u8;
        Self((sector + 1) % 4)
    }

    #[inline]
    #[must_use]
    /// Computes the counter clockwise [`EdgeDirection`] neighbor of self.
    ///
    /// # Example
    ///
    /// ```rust
    /// let diagonal = VertexDirection::RIGHT.direction_ccw();
    /// assert_eq!(diagonal, EdgeDirection::TOP_RIGHT);
    /// ```
    pub const fn direction_ccw(self) -> EdgeDirection {
        self.edge_ccw()
    }

    #[inline]
    #[must_use]
    /// Computes the counter clockwise [`EdgeDirection`] neighbor of self.
    ///
    /// # Example
    ///
    /// ```rust
    /// let diagonal = VertexDirection::RIGHT.edge_ccw();
    /// assert_eq!(diagonal, EdgeDirection::TOP_RIGHT);
    /// ```
    pub const fn edge_ccw(self) -> EdgeDirection {
        EdgeDirection(self.counter_clockwise().0)
    }

    #[inline]
    #[must_use]
    /// Computes the clockwise [`EdgeDirection`] neighbor of self.
    ///
    /// # Example
    ///
    /// ```rust
    /// let diagonal = VertexDirection::RIGHT.direction_cw();
    /// assert_eq!(diagonal, EdgeDirection::BOTTOM_RIGHT);
    /// ```
    pub const fn direction_cw(self) -> EdgeDirection {
        self.edge_cw()
    }

    #[inline]
    #[must_use]
    /// Computes the clockwise [`EdgeDirection`] neighbor of self.
    ///
    /// # Example
    ///
    /// ```rust
    /// let diagonal = VertexDirection::RIGHT.edge_cw();
    /// assert_eq!(diagonal, EdgeDirection::BOTTOM_RIGHT);
    // ```
    pub const fn edge_cw(self) -> EdgeDirection {
        EdgeDirection(self.0)
    }

    #[inline]
    #[must_use]
    /// Computes the two adjacent [`EdgeDirection`] in clockwise order
    pub const fn edge_directions(self) -> [EdgeDirection; 2] {
        [self.edge_ccw(), self.edge_cw()]
    }
}

impl From<VertexDirection> for Position {
    fn from(value: VertexDirection) -> Self {
        value.into_position()
    }
}

#[cfg(not(target_arch = "spirv"))]
impl Debug for VertexDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = self.into_position();
        f.debug_struct("VertexDirection")
            .field("index", &self.0)
            .field("x", &c.x)
            .field("y", &c.y)
            .field("z", &c.z)
            .finish()
    }
}
