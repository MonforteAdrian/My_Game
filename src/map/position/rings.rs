use super::{iter::ExactSizePositionIterator, EdgeDirection, Position, VertexDirection};
use std::collections::HashSet;

impl Position {
    // TODO might be good to precompute the most used rings and store them in an array in some resource(ex: fov ring)
    // TODO impl cone
    #[must_use]
    #[allow(clippy::cast_possible_wrap)]
    #[allow(clippy::missing_panics_doc)]
    /// Retrieves one [`Position`] ring around `self` in a given `range`.
    pub fn ring(self, range: u32) -> impl ExactSizeIterator<Item = Self> {
        let mut circle_points = HashSet::new();

        let mut x = 0;
        let mut y = range as i32;
        let mut d = 3 - 2 * range as i32;

        while x <= y {
            // Add all 8 symmetric points
            circle_points.insert(Self { x: (self.x + x), y: (self.y + y), z: self.z });
            circle_points.insert(Self { x: (self.x - x), y: (self.y + y), z: self.z });
            circle_points.insert(Self { x: (self.x + x), y: (self.y - y), z: self.z });
            circle_points.insert(Self { x: (self.x - x), y: (self.y - y), z: self.z });
            circle_points.insert(Self { x: (self.x + y), y: (self.y + x), z: self.z });
            circle_points.insert(Self { x: (self.x - y), y: (self.y + x), z: self.z });
            circle_points.insert(Self { x: (self.x + y), y: (self.y - x), z: self.z });
            circle_points.insert(Self { x: (self.x - y), y: (self.y - x), z: self.z });

            // Update decision parameter and coordinates
            if d < 0 {
                d += 4 * x + 6;
            } else {
                d += 4 * (x - y) + 10;
                y -= 1;
            }
            x += 1;
        }
        let count = circle_points.len();
        let iter = circle_points.into_iter();
        ExactSizePositionIterator { iter, count }
    }
}
