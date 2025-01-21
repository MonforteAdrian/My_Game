#![allow(clippy::inline_always)]

use bevy::prelude::*;
use std::{collections::HashSet, fmt::Debug};

/// Collection of algorithms
mod algorithms;
/// Type conversions
mod convert;
/// Neighbors utilites
mod direction;
/// Traits implementations
mod impls;
/// Iterator tools module
mod iter;
#[cfg(test)]
mod tests;
/// Collections of utils
mod utils;

pub(crate) use algorithms::*;
pub use direction::*;
pub(crate) use iter::ExactSizePositionIterator;

/// Position Coordinates
#[derive(Component, Debug, Copy, Clone, Eq, Default, PartialEq, Hash)]
pub struct Position {
    /// Position in the x coordinate (bottom-left to top-right)
    pub x: i32,
    /// Position in the y coordinate (top-left to bottom-right)
    pub y: i32,
    /// Position in the z coordinate (depth)
    pub z: i32,
}

#[inline(always)]
#[must_use]
/// Instantiates a new position from axial coordinates
///
/// # Example
///
/// ```
/// let coord = position(3, 5, 0);
/// assert_eq!(coord.x, 3);
/// assert_eq!(coord.y, 5);
/// assert_eq!(coord.z, 0);
/// ```
pub const fn position(x: i32, y: i32, z: i32) -> Position {
    Position::new(x, y, z)
}

impl Position {
    /// (0, 0, 0)
    pub const ZERO: Self = Self::new(0, 0, 0);

    /// Positionmetric direct neighbor coordinates array
    ///
    /// ```txt
    ///          /\
    ///         /  \
    ///        /\  /\
    ///   +Y  /1 \/0 \ +X
    ///      /\  /\  /\
    ///     /  \/  \/  \
    ///     \  /\  /\  /
    ///      \/2 \/3 \/
    ///   -X  \  /\  / -Y
    ///        \/  \/
    ///         \  /
    ///          \/
    /// ```
    pub const NEIGHBORS_COORDS: [Self; 4] =
        [Self::new(1, 0, 0), Self::new(0, 1, 0), Self::new(-1, 0, 0), Self::new(0, -1, 0)];

    /// Positionmetric neighbor coordinates array
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
    pub const ALL_NEIGHBORS_COORDS: [Self; 8] = [
        Self::new(1, 0, 0),
        Self::new(1, 1, 0),
        Self::new(0, 1, 0),
        Self::new(-1, 1, 0),
        Self::new(-1, 0, 0),
        Self::new(-1, -1, 0),
        Self::new(0, -1, 0),
        Self::new(1, -1, 0),
    ];

    #[inline(always)]
    #[must_use]
    /// Instantiates a new Position from coordinates
    ///
    /// # Example
    ///
    /// ```
    /// let coord = Position::new(3, 5, 0);
    /// assert_eq!(coord.x, 3);
    /// assert_eq!(coord.y, 5);
    /// assert_eq!(coord.z, 0);
    /// ```
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    #[inline]
    #[must_use]
    /// Retrieves all 4 direct neighbor coordinates around `self`
    pub fn neighbors(self) -> [Self; 4] {
        Self::NEIGHBORS_COORDS.map(|n| self.const_add(n))
    }

    #[inline]
    #[must_use]
    /// Retrieves all 8 neighbor coordinates around `self`
    pub fn all_neighbors(self) -> [Self; 8] {
        Self::ALL_NEIGHBORS_COORDS.map(|n| self.const_add(n))
    }
}
