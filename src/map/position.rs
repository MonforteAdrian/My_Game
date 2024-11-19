use bevy::prelude::Component;

mod rings;

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
    /// (0, 0)
    pub const ORIGIN: Self = Self::ZERO;
    /// (0, 0)
    pub const ZERO: Self = Self::new(0, 0, 0);
    /// (1, 1)
    pub const ONE: Self = Self::new(1, 1, 0);
    /// (-1, -1)
    pub const NEG_ONE: Self = Self::new(-1, -1, 0);

    /// +X (1, 0)
    pub const X: Self = Self::new(1, 0, 0);
    /// -X (-1, 0)
    pub const NEG_X: Self = Self::new(-1, 0, 0);
    /// +Y (0, 1)
    pub const Y: Self = Self::new(0, 1, 0);
    /// -Y (0, -1)
    pub const NEG_Y: Self = Self::new(0, -1, 0);

    // TODO review
    /// Unit vectors that increase the X axis in clockwise order
    pub const INCR_X: [Self; 2] = [Self::new(1, 0, 0), Self::new(1, -1, 0)];
    /// Unit vectors that increase the Y axis in clockwise order
    pub const INCR_Y: [Self; 2] = [Self::new(0, 1, 0), Self::new(-1, 1, 0)];
    /// Unit vectors that increase the Z axis in clockwise order
    pub const INCR_Z: [Self; 2] = [Self::new(0, 0, 0), Self::new(0, 0, 0)];

    /// Unit vectors that decrease the X axis in clockwise order
    pub const DECR_X: [Self; 2] = [Self::new(-1, 0, 0), Self::new(-1, 1, 0)];
    /// Unit vectors that decrease the Y axis in clockwise order
    pub const DECR_Y: [Self; 2] = [Self::new(0, -1, 0), Self::new(1, -1, 0)];
    /// Unit vectors that decrease the Z axis in clockwise order
    pub const DECR_Z: [Self; 2] = [Self::new(0, 0, 0), Self::new(0, 0, 0)];

    /// Isometric edge neighbor coordinates array, following [`EdgeDirection`]
    /// order
    ///
    /// ```txt
    ///        /\  /\
    ///   -X  /2 \/3 \ +Y
    ///       \  /\  /
    ///        \/  \/
    ///        /\  /\
    ///       /1 \/0 \
    ///   -Y \  /\  / +X
    ///       \/  \/
    /// ```
    ///
    /// Cubic coordinates:
    ///
    /// ```txt
    ///            /\  /\
    ///   (-1, 0) /2 \/3 \ (0, 1)
    ///           \  /\  /
    ///            \/  \/
    ///            /\  /\
    ///   (0, -1) /1 \/0 \ (1, 0)
    ///          \  /\  /
    ///           \/  \/
    /// ```
    pub const NEIGHBORS_COORDS: [Self; 4] = [
        Self::new(1, 0, 0),
        Self::new(0, -1, 0),
        Self::new(-1, 0, 0),
        Self::new(0, 1, 0),
    ];

    /// Isometric diagonal neighbor coordinates array, following
    /// [`VertexDirection`] order
    ///
    /// ```txt
    ///          /\
    ///         /2 \
    ///        /\  /\
    ///   -X  /  \/  \ +Y
    ///      /\  /\  /\
    ///     /1 \/  \/3 \
    ///     \  /\  /\  /
    ///      \/  \/  \/
    ///   -Y  \  /\  / +X
    ///        \/0 \/
    ///         \  /
    ///          \/
    /// ```
    pub const DIAGONAL_COORDS: [Self; 4] = [
        Self::new(1, -1, 0),
        Self::new(-1, -1, 0),
        Self::new(-1, 1, 0),
        Self::new(1, 1, 0),
    ];

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
    #[allow(clippy::cast_precision_loss)]
    /// Converts `self` to an [`f32`] array as `[x, y, z]`
    pub const fn to_array_f32(self) -> [f32; 3] {
        [self.x as f32, self.y as f32, self.z as f32]
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
    /// ```rust
    /// let point = [0.6, 10.2, 0.5];
    /// let coord = Position::round(point);
    /// assert_eq!(coord.x, 1);
    /// assert_eq!(coord.y, 10);
    /// assert_eq!(coord.z, 1);
    /// ```
    pub fn round([mut x, mut y, z]: [f32; 3]) -> Self {
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
    /// Computes the distance from `self` to `rhs` as an unsigned integer
    ///
    /// See [`Self::distance_to`] for the signed version
    pub const fn unsigned_distance_to(self, rhs: Self) -> u32 {
        self.const_sub(rhs).ulength()
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
    /// ```rust
    /// let coord = Position::new(10, 0);
    /// assert_eq!(coord.ulength(), 10);
    /// ```
    pub const fn ulength(self) -> u32 {
        let [x, y, z] = [
            self.x.unsigned_abs(),
            self.y.unsigned_abs(),
            self.z.unsigned_abs(),
        ];
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
    /// Retrieves all 4 neighbor coordinates around `self`
    pub fn all_neighbors(self) -> [Self; 4] {
        Self::NEIGHBORS_COORDS.map(|n| self.const_add(n))
    }

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
}
