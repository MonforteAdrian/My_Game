// TODO this might be useful in the future if I want different projections otherwise simply replace the call in layout with the formula
/// Isometric matrices and offset
const ISO_TILE_POSITION_MATRIX: ProjectionMatrix = ProjectionMatrix {
    forward_matrix: [0.5, 0.25, -0.5, 0.25, 0.0, 0.5],
};

/// Matrix for isometric projection
#[derive(Debug, Clone, PartialEq)]
pub struct ProjectionMatrix {
    /// Matrix used to compute isometric coordinates to world/pixel coordinates
    pub(crate) forward_matrix: [f32; 6],
}

impl Default for ProjectionMatrix {
    fn default() -> Self {
        Self {
            forward_matrix: ISO_TILE_POSITION_MATRIX.forward_matrix,
        }
    }
}

impl ProjectionMatrix {
    #[must_use]
    #[inline]
    /// Applies `matrix` to a point defined by `x`, `y` and `z`
    fn matrix_op(matrix: [f32; 6], [x, y, z]: [f32; 3]) -> [f32; 3] {
        [
            x.mul_add(matrix[0], y.mul_add(matrix[2], matrix[4] * z)),
            x.mul_add(matrix[1], y.mul_add(matrix[3], matrix[5] * z)),
            z,
        ]
    }

    #[must_use]
    #[inline]
    /// Applies the matrix `forward_matrix` to a point `p`
    pub fn forward(&self, point: [f32; 3]) -> [f32; 3] {
        Self::matrix_op(self.forward_matrix, point)
    }
}

// TODO create tests
#[cfg(test)]
mod tests {
    use super::*;
}
