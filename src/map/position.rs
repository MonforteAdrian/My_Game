#![allow(clippy::inline_always)]
use crate::{DirectionWay, EdgeDirection, VertexDirection};

use bevy::prelude::*;
use std::{collections::HashSet, fmt::Debug};

/// Type conversions
mod convert;
/// Traits implementations
mod impls;
/// Iterator tools module
mod iter;
/// Position ring utils
mod rings;
#[cfg(test)]
mod tests;
/// Collections of utils
mod utils;

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

impl Position {
    /// (0, 0, 0)
    pub const ZERO: Self = Self::new(0, 0, 0);

    /// +X (1, 0, 0)
    pub const X: Self = Self::new(1, 0, 0);
    /// -X (-1, 0, 0)
    pub const NEG_X: Self = Self::new(-1, 0, 0);
    /// +Y (0, 1, 0)
    pub const Y: Self = Self::new(0, 1, 0);
    /// -Y (0, -1, 0)
    pub const NEG_Y: Self = Self::new(0, -1, 0);
    /// +Z (0, 0, 1)
    pub const Z: Self = Self::new(0, 0, 1);
    /// -Z (0, 0, -1)
    pub const NEG_Z: Self = Self::new(0, 0, -1);

    /// Positionmetric edge neighbor coordinates array, following [`EdgeDirection`]
    /// order
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
    /// ```
    ///
    /// Cubic coordinates:
    ///
    /// ```txt
    ///            /\  /\
    ///    (0, 1) /3 \/0 \ (1, 0)
    ///           \  /\  /
    ///            \/  \/
    ///            /\  /\
    ///    (-1,0) /2 \/1 \ (0,-1)
    ///           \  /\  /
    ///            \/  \/
    /// ```
    pub const NEIGHBORS_COORDS: [Self; 4] =
        [Self::new(1, 0, 0), Self::new(0, -1, 0), Self::new(-1, 0, 0), Self::new(0, 1, 0)];

    /// Positionmetric diagonal neighbor coordinates array, following
    /// [`VertexDirection`] order
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
    pub const DIAGONAL_COORDS: [Self; 4] =
        [Self::new(1, 1, 0), Self::new(1, -1, 0), Self::new(-1, -1, 0), Self::new(-1, 1, 0)];

    #[inline(always)]
    #[must_use]
    /// Instantiates a new Position from coordinates
    ///
    /// # Example
    ///
    /// ```rust
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
    /// Instantiates a new Position tile with all coordinates set to `v`
    ///
    /// # Example
    ///
    /// ```rust
    /// let coord = Position::splat(3);
    /// assert_eq!(coord.x, 3);
    /// assert_eq!(coord.y, 3);
    /// assert_eq!(coord.z, 3);
    /// ```
    pub const fn splat(v: i32) -> Self {
        Self { x: v, y: v, z: v }
    }

    #[inline]
    #[must_use]
    /// Retrieves all 4 neighbor coordinates around `self`
    pub fn all_neighbors(self) -> [Self; 4] {
        Self::NEIGHBORS_COORDS.map(|n| self.const_add(n))
    }

    #[inline]
    #[must_use]
    /// Retrieves all 4 neighbor diagonal coordinates around `self`
    pub fn all_diagonals(self) -> [Self; 4] {
        Self::DIAGONAL_COORDS.map(|n| self.const_add(n))
    }

    #[must_use]
    /// Find in which [`VertexDirection`] wedge `rhs` is relative to `self`
    pub fn diagonal_way_to(self, rhs: Self) -> DirectionWay<VertexDirection> {
        let [x, y, z] = (rhs - self).to_array_f32();
        let [xa, ya, za] = [x.abs(), y.abs(), z.abs()];
        match xa.max(ya).max(za) {
            // TODO this VertexDirection might be wrong
            v if v == xa => DirectionWay::way_from(x < 0., xa == ya, xa == za, VertexDirection::TOP),
            // TODO remove this when 3d
            _ => DirectionWay::way_from(y < 0., ya == za, ya == xa, VertexDirection::BOTTOM),
            //v if v == ya => {
            //    DirectionWay::way_from(y < 0., ya == za, ya == xa, VertexDirection::LEFT)
            //}
            // TODO once we do edge and vertex in 3d this should be up/down
            //_ => DirectionWay::way_from(z < 0., za == xa, za == ya, VertexDirection::BOTTOM),
        }
    }

    #[must_use]
    /// Find in which [`EdgeDirection`] wedge `rhs` is relative to `self`
    pub fn way_to(self, rhs: Self) -> DirectionWay<EdgeDirection> {
        let [x, y, z] = (rhs - self).to_array_f32();
        let [x, y, z] = [y - x, z - y, x - z];
        let [xa, ya, za] = [x.abs(), y.abs(), z.abs()];
        match xa.max(ya).max(za) {
            v if v == xa => DirectionWay::way_from(x < 0., xa == ya, xa == za, EdgeDirection::BOTTOM_RIGHT),
            // TODO remove this when 3d
            _ => DirectionWay::way_from(z < 0., za == xa, za == ya, EdgeDirection::TOP_RIGHT),
            //v if v == ya => {
            //    DirectionWay::way_from(y < 0., ya == za, ya == xa, EdgeDirection::TOP_LEFT)
            //}
            // TODO once we do edge and vertex in 3d this should be up/down
            //_ => DirectionWay::way_from(z < 0., za == xa, za == ya, EdgeDirection::BOTTOM_LEFT),
        }
    }

    #[allow(clippy::cast_precision_loss)]
    #[must_use]
    /// Computes all coordinates in a line from `self` to `other`.
    ///
    /// # Example
    /// ```
    /// let start = Position::ZERO;
    /// let end = Position::new(5, 0, 0);
    ///
    /// let line = start.line_to(end);
    /// let line: Vec<Position> = line.collect();
    /// ````
    pub fn line_to(self, other: Self) -> impl ExactSizeIterator<Item = Self> {
        // TODO this is sometime way better and others way worse
        // maybe when you put profiling you can improve it
        //let d = other - self;
        //let n = d.abs();
        //let sign = d.signum();

        //let mut positions = Vec::new();
        //let mut seen = HashSet::new();
        //let mut p = self;
        //positions.push(p);
        //seen.insert(p);

        //let mut ix = 0;
        //let mut iy = 0;

        //while ix < n.x || iy < n.y {
        //    if (ix * n.y + n.y / 2) < (iy * n.x + n.x / 2) {
        //        // next step is horizontal
        //        p.x += sign.x;
        //        ix += 1;
        //    } else {
        //        // next step is vertical
        //        p.y += sign.y;
        //        iy += 1;
        //    }
        //    if seen.insert(p) {
        //        positions.push(p);
        //    }
        //}

        //ExactSizePositionIterator {
        //    count: positions.len(),
        //    iter: positions.into_iter(),
        //}

        let d = other - self;
        let n = d.abs();
        let sign = d.signum();

        let count = (n.x.max(n.y) + 1) as usize;
        let mut p = self;

        let mut ix = 0;
        let mut iy = 0;
        let iter = std::iter::once(p).chain(std::iter::from_fn(move || {
            if ix >= n.x && iy >= n.y {
                return None;
            }
            if (ix * n.y + n.y / 2) < (iy * n.x + n.x / 2) {
                // next step is horizontal
                p.x += sign.x;
                ix += 1;
            } else {
                // next step is vertical
                p.y += sign.y;
                iy += 1;
            }
            Some(p)
        }));
        ExactSizePositionIterator { iter, count }
    }

    #[inline]
    #[must_use]
    /// Retrieves the diagonal neighbor coordinates matching the given
    /// `direction`
    pub const fn diagonal_neighbor_coord(direction: VertexDirection) -> Self {
        direction.into_position()
    }

    #[inline]
    #[must_use]
    /// Retrieves the neighbor coordinates matching the given
    /// `direction`
    pub const fn neighbor_coord(direction: EdgeDirection) -> Self {
        direction.into_position()
    }

    pub(crate) const fn add_dir(self, direction: EdgeDirection) -> Self {
        self.const_add(Self::neighbor_coord(direction))
    }

    pub(crate) const fn add_diag_dir(self, direction: VertexDirection) -> Self {
        self.const_add(Self::diagonal_neighbor_coord(direction))
    }
}
