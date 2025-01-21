use crate::Position;
use std::sync::LazyLock;

/// Maximum radius to precompute rings for.
pub const MAX_RADIUS: usize = 201;

#[allow(clippy::type_complexity)]
// Precomputed offsets for radiuss 0 to 200
pub static PRECOMPUTED_RINGS: LazyLock<[Option<Vec<Position>>; MAX_RADIUS]> = LazyLock::new(|| {
    let mut rings: [Option<Vec<Position>>; MAX_RADIUS] = [const { None }; MAX_RADIUS];
    for radius in 0..MAX_RADIUS as i32 {
        let mut points = Vec::new();
        let mut x = 0;
        let mut y = radius;
        let z = 0;
        let mut d = 3 - 2 * radius;

        while x <= y {
            points.extend_from_slice(&generate_symmetric_points(x, y, z));
            if d < 0 {
                d += 4 * x + 6;
            } else {
                d += 4 * (x - y) + 10;
                y -= 1;
            }
            x += 1;
        }

        // Sort the points in circular order based on their angle
        points.sort_by(|a, b| a.angle().partial_cmp(&b.angle()).unwrap_or(std::cmp::Ordering::Equal));
        points.dedup();
        rings[radius as usize] = Some(points);
    }
    rings
});

pub const fn generate_symmetric_points(x: i32, y: i32, z: i32) -> [Position; 8] {
    [
        Position { x, y, z },
        Position { x: -x, y, z },
        Position { x, y: -y, z },
        Position { x: -x, y: -y, z },
        Position { x: y, y: x, z },
        Position { x: -y, y: x, z },
        Position { x: y, y: -x, z },
        Position { x: -y, y: -x, z },
    ]
}
