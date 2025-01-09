use crate::{EdgeDirection, Position};
use std::collections::HashSet;

/// Computes a field of view around `coord` in a given `range`
///
/// This algorithm takes in account coordinates *visibility* through the
/// `blocking` argument. (*Blocking* coordinates should return `true`)
///
/// # Examples
///
/// - Compute field of view with no boundaries and some blocking tiles
///
/// ```rust
/// # use std::collections::HashSet;
/// use crate::map::algorithms::range_fov;
///
/// let pos = Position{0, 0, 0};
/// let range = 10;
/// let blocking_coords: HashSet<Position> = HashSet::new();
/// // Add blocking coordinates
/// // blocking_coords.insert(hex(2, 0));
/// // ..
/// let fov = range_fov(pos, range, |h| blocking_coords.contains(&h));
/// ```
pub fn range_fov(coord: Position, range: u32, blocking: impl Fn(Position) -> bool) -> HashSet<Position> {
    coord.ring(range).flat_map(|target| coord.line_to(target).take_while(|h| !blocking(*h))).collect()
}

/// Computes a field of view around `coord` in a given `range` towards
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
/// ```rust
/// # use std::collections::HashSet;
/// use crate::map::algorithms::directional_fov;
///
/// let pos = Position{0, 0, 0};
/// let range = 10;
/// let dir = EdgeDirection::TOP_RIGHT;
/// let blocking_coords: HashSet<Position> = HashSet::new();
/// // Add blocking coordinates
/// // blocking_coords.insert(hex(2, 0));
/// // ..
/// let fov = directional_fov(pos, range, dir, |h| blocking_coords.contains(&h));
/// ```
pub fn directional_fov(
    coord: Position,
    range: u32,
    direction: EdgeDirection,
    blocking: impl Fn(Position) -> bool,
) -> HashSet<Position> {
    let [a, b] = direction.vertex_directions();
    coord
        .ring(range)
        .filter(|h| {
            let way = coord.diagonal_way_to(*h);
            way == a || way == b
        })
        .flat_map(|target| coord.line_to(target).take_while(|h| !blocking(*h)))
        .collect()
}
