use super::PRECOMPUTED_RINGS;
use crate::{ExactSizePositionIterator, Position};

impl Position {
    #[must_use]
    /// Retrieves one [`Position`] ring around `self` in a given `radius`.
    pub fn ring(self, radius: u32) -> impl ExactSizeIterator<Item = Self> {
        PRECOMPUTED_RINGS[radius as usize]
            .as_ref()
            .expect("radius out of bounds")
            .iter()
            .map(move |&offset| self + offset)
    }

    #[must_use]
    /// Retrieves points within a cone defined by `start_angle` and `end_angle` in a given `radius` ring.
    pub fn cone(self, radius: u32, direction: f32, angle: f32) -> impl Iterator<Item = Self> {
        let ring = PRECOMPUTED_RINGS[radius as usize].as_ref().expect("radius out of bounds");
        let ring_len = ring.len() as i32;

        // Convert angles to fixed-point (scaled integers)
        let direction_fixed = ((direction * ring_len as f32 / (2.0 * std::f32::consts::PI)) as i32) % ring_len;
        let angle_fixed = ((angle * ring_len as f32 / (2.0 * std::f32::consts::PI)) as i32).max(1);

        (0..angle_fixed).map(move |i| {
            let idx = (direction_fixed - angle_fixed / 2 + i + ring_len) % ring_len;
            self + ring[idx as usize]
        })
    }

    // TODO when we do 3d
    // spheres
    // spherical sector
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
}
