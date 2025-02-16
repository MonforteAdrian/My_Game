use crate::{fov, Creature, Direction, IsoGrid, Position, Viewshed};
use bevy::prelude::{Changed, Query, Res, Transform, With};

#[allow(clippy::type_complexity)]
pub fn field_of_view_system(
    mut query: Query<(&Position, &mut Viewshed, &Direction), (With<Creature>, Changed<Transform>)>,
    grid: Res<IsoGrid>,
) {
    // This should only be triggered when the creature moves, either to another tile or facing direction
    for (pos, mut viewshed, direction) in query.iter_mut() {
        viewshed.visible_tiles = fov(
            *pos,
            viewshed.range,
            *direction,
            (viewshed.angle as f32).to_radians(),
            |h| grid.blocked_coords.contains(&h) || !grid.tiles.contains_key(&h),
        );
    }
}
