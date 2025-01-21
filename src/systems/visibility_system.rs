use crate::{fov, Creature, Direction, IsoGrid, Position, Viewshed};
use bevy::prelude::{Changed, Query, ResMut, Transform, With};

pub fn visibility_system(
    // TODO when facing direction is implemented added to the filters with or
    mut query: Query<(&mut Position, &mut Viewshed, &Direction), (With<Creature>, Changed<Transform>)>,
    grid: ResMut<IsoGrid>,
) {
    // This should only be triggered when the creature moves, either to another tile or facing direction
    for (pos, mut viewshed, direction) in query.iter_mut() {
        viewshed.visible_tiles =
            fov(*pos, viewshed.range, *direction, (viewshed.angle as f32).to_radians().into(), |h| {
                grid.blocked_coords.contains(&h) || !grid.entities.contains_key(&h)
            })
    }
}
