use crate::{Direction, Position};
use std::collections::HashSet;

/// Computes a field of view around `coord` in a given `radius` towards
/// `direction` with 120 degrees vision
///
/// This algorithm takes in account coordinates *visibility* through the
/// `blocking` argument. (*Blocking* coordinates should return `true`)
///
/// # Examples
///
/// - Compute drectional field of view with no boundaries and some blocking
///   tiles
///
/// ```
/// let pos = Position{0, 0, 0};
/// let radius = 10;
/// let dir = Direction::NorthEast;
/// let angle = 120;
/// let blocking_coords: HashSet<Position> = HashSet::new();
/// // Add blocking coordinates
/// // blocking_coords.insert(hex(2, 0));
/// // ..
/// let fov = fov(pos, radius, dir, angle, |h| blocking_coords.contains(&h));
/// ```
pub fn fov(
    coord: Position,
    radius: u32,
    direction: Direction,
    angle: f32,
    blocking: impl Fn(Position) -> bool,
) -> HashSet<Position> {
    coord
        .cone(radius, direction.angle(), angle)
        .flat_map(|target| coord.line_to(target).take_while(|h| !blocking(*h)))
        .collect()
}
