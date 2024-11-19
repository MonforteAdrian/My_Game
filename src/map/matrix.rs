/// Isometric matrices and offset
const ISO_TILE_POSITION_MATRIX: ProjectionMatrix = ProjectionMatrix {
    forward_matrix: [0.5, 0.25, -0.5, 0.25, 0.0, 0.5],
    inverse_matrix: [1.0, -1.0, 2.0, 2.0, -1.0, -1.0],
};

/// Matrix for isometric projection
#[derive(Debug, Clone, PartialEq)]
pub struct ProjectionMatrix {
    /// Matrix used to compute isometric coordinates to world/pixel coordinates
    pub(crate) forward_matrix: [f32; 6],
    /// Matrix used to compute world/pixel coordinates to isometric coordinates
    pub(crate) inverse_matrix: [f32; 6],
}

impl Default for ProjectionMatrix {
    fn default() -> Self {
        Self {
            forward_matrix: ISO_TILE_POSITION_MATRIX.forward_matrix,
            inverse_matrix: ISO_TILE_POSITION_MATRIX.inverse_matrix,
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

    #[must_use]
    #[inline]
    /// Applies the matrix `inverse_matrix` to a point `p`
    pub fn inverse(&self, point: [f32; 3]) -> [f32; 3] {
        Self::matrix_op(self.inverse_matrix, point)
    }
}

// TODO run and make sure this works
#[cfg(test)]
mod tests {
    use super::*;

    /// Helper function to assert approximate equality of floating-point arrays.
    fn assert_approx_eq(expected: [f32; 3], actual: [f32; 3], tolerance: f32) {
        assert!(
            (expected[0] - actual[0]).abs() < tolerance
                && (expected[1] - actual[1]).abs() < tolerance
                && (expected[2] - actual[2]).abs() < tolerance,
            "Expected: {:?}, got: {:?}, within tolerance: {}",
            expected,
            actual,
            tolerance
        );
    }

    #[test]
    fn test_forward_identity() {
        let matrix = ProjectionMatrix::default();
        let point = [0.0, 0.0, 0.0];
        let expected = [0.0, 0.0, 0.0];
        let transformed_point = matrix.forward(point);
        assert_approx_eq(expected, transformed_point, 0.00001);
    }

    #[test]
    fn test_inverse_identity() {
        let matrix = ProjectionMatrix::default();
        let point = [0.0, 0.0, 0.0];
        let expected = [0.0, 0.0, 0.0];
        let transformed_point = matrix.inverse(point);
        assert_approx_eq(expected, transformed_point, 0.00001);
    }

    #[test]
    fn test_forward_and_inverse() {
        let matrix = ProjectionMatrix::default();
        let points = [
            [1.0, 2.0, 3.0],
            [-1.0, -2.0, -3.0],
            [100.0, 200.0, 300.0],
            [0.001, 0.002, 0.003],
        ];

        for &point in points.iter() {
            let transformed_point = matrix.forward(point);
            let recovered_point = matrix.inverse(transformed_point);
            assert_approx_eq(point, recovered_point, 0.00001);
        }
    }

    #[test]
    fn test_high_value_transformation() {
        let matrix = ProjectionMatrix::default();
        let point = [1e6, 1e6, 1e6];
        let transformed_point = matrix.forward(point);
        let recovered_point = matrix.inverse(transformed_point);
        assert_approx_eq(point, recovered_point, 0.1);
    }
}
