use super::Position;
use std::f32::consts::TAU;

impl Position {
    #[inline]
    #[must_use]
    /// adds `self` and `other`.
    ///
    /// [`Position`] implements [`Add`] (`+` operator) but this method is `const`.
    ///
    /// [`Add`]: std::ops::Add
    pub const fn const_add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    #[inline]
    #[must_use]
    /// substracts `self` and `rhs`.
    ///
    /// [`Position`] implements [`Sub`] (`-` operator) but this method is `const`.
    ///
    /// [`Sub`]: std::ops::Sub
    pub const fn const_sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }

    #[inline]
    #[must_use]
    /// Negates the coordinate, giving its reflection (symmetry) around the
    /// origin.
    ///
    /// [`Position`] implements [`Neg`] (`-` operator) but this method is `const`.
    ///
    /// [`Neg`]: std::ops::Neg
    pub const fn const_neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: self.z,
        }
    }

    /// Shift constant used for [Position] operations
    #[inline]
    #[must_use]
    pub(crate) const fn shift(range: u32) -> u32 {
        // TODO test this and make sure is correct
        3 * range + 2
    }

    #[inline]
    #[must_use]
    /// Computes the absolute value of `self`
    ///
    /// # Example
    ///
    /// ```
    /// let coord = Position::new(-1, -32, -5).abs();
    /// assert_eq!(coord.x, 1);
    /// assert_eq!(coord.y, 32);
    /// assert_eq!(coord.z, 5);
    /// ```
    pub const fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    #[inline]
    #[must_use]
    /// Returns a [`Position`] with elements representing the sign of `self`.
    ///
    ///  - `0` if the number is zero
    ///  - `1` if the number is positive
    ///  - `-1` if the number is negative
    pub const fn signum(self) -> Self {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
            z: self.z.signum(),
        }
    }

    #[inline]
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    /// Rounds floating point coordinates to [`Position`].
    /// This method is used for operations like multiplications and divisions
    /// with floating point numbers.
    ///
    /// # Example
    ///
    /// ```
    /// let point = [0.6, 10.2, 0.5];
    /// let coord = Position::round(point);
    /// assert_eq!(coord.x, 1);
    /// assert_eq!(coord.y, 10);
    /// assert_eq!(coord.z, 1);
    /// ```
    pub fn round([mut x, mut y, z]: [f32; 3]) -> Self {
        // TODO add rounding for z?
        let [mut x_r, mut y_r] = [x.round(), y.round()];
        x -= x_r;
        y -= y_r;
        if x.abs() >= y.abs() {
            x_r += 0.5_f32.mul_add(y, x).round();
        } else {
            y_r += 0.5_f32.mul_add(x, y).round();
        }
        Self::new(x_r as i32, y_r as i32, z as i32)
    }

    #[inline]
    #[must_use]
    /// Calculates the angle in radians of [`Position`].
    ///
    /// # Example
    ///
    /// ```
    /// let point = position(1,0,0);
    /// let angle = point.angle();
    /// assert_eq!(angle, 0);
    /// ```
    pub fn angle(&self) -> f32 {
        ((self.y as f32).atan2(self.x as f32) + TAU) % TAU
    }

    #[inline]
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    /// Converts `self` to an [`f32`] array as `[x, y, z]`
    pub const fn to_array_f32(self) -> [f32; 3] {
        [self.x as f32, self.y as f32, self.z as f32]
    }

    #[inline]
    #[must_use]
    #[doc(alias = "unsigned_length")]
    /// Computes coordinates length as an unsigned integer
    /// The length of a [`Position`] coordinate is equal to its distance from the
    /// origin.
    ///
    /// See [`Self::length`] for the signed version
    ///
    /// # Example
    /// ```
    /// let coord = Position::new(10, 0);
    /// assert_eq!(coord.ulength(), 10);
    /// ```
    pub const fn ulength(self) -> u32 {
        let [x, y, z] = [self.x.unsigned_abs(), self.y.unsigned_abs(), self.z.unsigned_abs()];
        if x >= y && x >= z {
            x
        } else if y >= x && y >= z {
            y
        } else {
            z
        }
    }

    #[inline]
    #[must_use]
    /// Computes the distance from `self` to `rhs` as an
    /// unsigned integer
    ///
    /// See [`Self::distance_to`] for the signed version
    pub const fn unsigned_distance_to(self, rhs: Self) -> u32 {
        self.const_sub(rhs).ulength()
    }

    #[inline]
    #[must_use]
    /// Computes the neighbor direction from `self` to `rhs`
    pub fn direction_to_neighbor(self, rhs: Self) -> Option<u8> {
        self.all_neighbors()
            .iter()
            .position(|&neighbor| neighbor == rhs)
            .map(|i| i as u8)
    }
}
