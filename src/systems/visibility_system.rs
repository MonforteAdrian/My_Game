use crate::{fov, Creature, Direction, EntityName, IsoGrid, Position, Viewshed};
use bevy::prelude::{Changed, Query, Res, Transform, With};

pub fn visibility_system(
    mut query: Query<(&Position, &mut Viewshed, &Direction), (With<Creature>, Changed<Transform>)>,
    grid: Res<IsoGrid>,
) {
    // This should only be triggered when the creature moves, either to another tile or facing direction
    for (pos, mut viewshed, direction) in query.iter_mut() {
        viewshed.visible_tiles = fov(
            *pos,
            viewshed.range,
            *direction,
            (viewshed.angle as f32).to_radians().into(),
            |h| grid.blocked_coords.contains(&h) || !grid.tiles.contains_key(&h),
        );
        for entity_pos in grid.entities.keys() {
            if viewshed.visible_tiles.contains(entity_pos) && entity_pos != pos {
                //dbg!(name, "found and entity");
            }
        }
    }
}
