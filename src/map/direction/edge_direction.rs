use super::{
    angles::{DIRECTION_ANGLE_DEGREES, DIRECTION_ANGLE_RAD},
    VertexDirection,
};
use crate::Position;
use bevy::prelude::Vec2;
use std::fmt::Debug;

/// All 4 possible neighbor/edge directions in isometric space.
///
/// ```txt
///        /\  /\
///   +Y  /3 \/0 \ +X
///       \  /\  /
///        \/  \/
///        /\  /\
///       /2 \/1 \
///   -X  \  /\  / -Y
///        \/  \/
/// ```txt
///
/// See [`Position::NEIGHBORS_COORDS`](crate::Position::NEIGHBORS_COORDS)
///
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
/// let direction = EdgeDirection::TOP_RIGHT;
/// assert_eq!(-direction, EdgeDirection::FLAT_BOTTOM);
/// assert_eq!(direction >> 1, EdgeDirection::FLAT_TOP_RIGHT);
/// assert_eq!(direction << 1, EdgeDirection::FLAT_TOP_LEFT);
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
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(not(target_arch = "spirv"), derive(Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
#[repr(transparent)]
#[doc(alias = "Direction")]
pub struct EdgeDirection(pub(crate) u8);

impl EdgeDirection {
    /// Direction towards `X`
    /// Direction to (1, 0)
    pub const X: Self = Self(0);
    ///
    /// Represents "Top Right" edge
    pub const TOP_RIGHT: Self = Self(0);
    ///
    /// Represents "North East" edge
    pub const NORTH_EAST: Self = Self(0);

    /// Direction towards `-Y`
    /// Direction to (0, -1)
    pub const NEG_Y: Self = Self(1);
    ///
    /// Represents "Bottom Right" edge
    pub const BOTTOM_RIGHT: Self = Self(1);
    ///
    /// Represents "South East" edge
    pub const SOUTH_EAST: Self = Self(1);

    /// Direction towards `-X`
    /// Direction to (-1, 0)
    pub const NEG_X: Self = Self(2);
    ///
    /// Represents "Bottom Left" edge
    pub const BOTTOM_LEFT: Self = Self(2);
    ///
    /// Represents "South West" edge
    pub const SOUTH_WEST: Self = Self(2);

    /// Direction towards `Y`
    /// Direction to (0, 1)
    pub const Y: Self = Self(3);
    ///
    /// Represents "Top Left" edge
    pub const TOP_LEFT: Self = Self(3);
    ///
    /// Represents "North West" edge
    pub const NORTH_WEST: Self = Self(3);

    /// All 4 hexagonal directions matching
    /// [`Position::NEIGHBORS_COORDS`](crate::Position::NEIGHBORS_COORDS)
    ///
    /// ```txt
    ///        /\  /\
    ///   +Y  /3 \/0 \ +X
    ///       \  /\  /
    ///        \/  \/
    ///        /\  /\
    ///       /2 \/1 \
    ///   -X  \  /\  / -Y
    ///        \/  \/
    /// ```txt
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

    /// Converts the direction to a normalized Position
    #[must_use]
    #[inline]
    pub const fn into_position(self) -> Position {
        Position::NEIGHBORS_COORDS[self.0 as usize]
    }

    /// Computes the opposite direction of `self`
    ///
    /// # Example
    ///
    /// ```rust
    /// assert_eq!(
    ///     EdgeDirection::TOP_RIGHT.const_neg(),
    ///     EdgeDirection::BOTTOM_LEFT
    /// );
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
    /// assert_eq!(
    ///     EdgeDirection::TOP_RIGHT.clockwise(),
    ///     EdgeDirection::BOTTOM_RIGHT
    /// );
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
    ///     EdgeDirection::TOP_RIGHT.counter_clockwise(),
    ///     EdgeDirection::TOP_LEFT
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
    /// assert_eq!(
    ///     EdgeDirection::TOP_RIGHT,
    ///     EdgeDirection::TOP_RIGHT.rotate_ccw(4)
    /// );
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
    /// assert_eq!(
    ///     EdgeDirection::TOP_RIGHT,
    ///     EdgeDirection::TOP_RIGHT.rotate_cw(4)
    /// );
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
    /// let direction = EdgeDirection::from_angle_degrees(35.0);
    /// assert_eq!(direction, EdgeDirection::FLAT_BOTTOM_RIGHT);
    /// ```
    pub fn from_angle_degrees(angle: f32) -> Self {
        let angle = angle.rem_euclid(360.0);
        let sector = (angle / DIRECTION_ANGLE_DEGREES).trunc() as u8;
        Self(sector)
    }

    // TODO fix this example and check this code
    #[must_use]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    /// Returns the direction from the given `angle` in radians
    ///
    /// # Example
    ///
    /// ```rust
    /// let direction = EdgeDirection::from_flat_angle(0.6);
    /// assert_eq!(direction, EdgeDirection::FLAT_BOTTOM_RIGHT);
    /// ```
    pub fn from_flat_angle(angle: f32) -> Self {
        let angle = angle.rem_euclid(std::f32::consts::TAU);
        let sector = (angle / DIRECTION_ANGLE_RAD) as u8;
        Self(sector)
    }

    // TODO fix this
    #[inline]
    #[must_use]
    /// Computes the counter clockwise [`VertexDirection`] neighbor of self.
    ///
    /// # Example
    ///
    /// ```rust
    /// let diagonal = EdgeDirection::FLAT_TOP.diagonal_ccw();
    /// assert_eq!(diagonal, VertexDirection::FLAT_TOP_LEFT);
    /// ```
    pub const fn diagonal_ccw(self) -> VertexDirection {
        self.vertex_ccw()
    }

    #[inline]
    #[must_use]
    /// Computes the counter clockwise [`VertexDirection`] neighbor of self.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// let diagonal = EdgeDirection::FLAT_TOP.vertex_ccw();
    /// assert_eq!(diagonal, VertexDirection::FLAT_TOP_LEFT);
    /// ```
    pub const fn vertex_ccw(self) -> VertexDirection {
        VertexDirection(self.0)
    }

    #[inline]
    #[must_use]
    /// Computes the clockwise [`VertexDirection`] neighbor of self.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// let diagonal = EdgeDirection::FLAT_TOP.diagonal_cw();
    /// assert_eq!(diagonal, VertexDirection::FLAT_TOP_RIGHT);
    /// ```
    pub const fn diagonal_cw(self) -> VertexDirection {
        self.vertex_cw()
    }

    #[inline]
    #[must_use]
    /// Computes the clockwise [`VertexDirection`] neighbor of self.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// let diagonal = EdgeDirection::FLAT_TOP.vertex_cw();
    /// assert_eq!(diagonal, VertexDirection::FLAT_TOP_RIGHT);
    /// ```
    pub const fn vertex_cw(self) -> VertexDirection {
        VertexDirection(self.clockwise().0)
    }

    #[inline]
    #[must_use]
    /// Computes the two adjacent [`VertexDirection`] in clockwise order
    pub const fn vertex_directions(self) -> [VertexDirection; 2] {
        [self.vertex_ccw(), self.vertex_cw()]
    }
}

impl From<EdgeDirection> for Position {
    fn from(value: EdgeDirection) -> Self {
        value.into_position()
    }
}

#[cfg(not(target_arch = "spirv"))]
impl Debug for EdgeDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = self.into_position();
        f.debug_struct("EdgeDirection")
            .field("index", &self.0)
            .field("x", &c.x)
            .field("y", &c.y)
            .field("z", &c.z)
            .finish()
    }
}
