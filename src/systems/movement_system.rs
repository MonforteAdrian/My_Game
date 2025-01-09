use crate::{Creature, IsoGrid, MoveTimer, PathfindingSteps, Position};
use bevy::prelude::{Query, Res, ResMut, Time, Transform, With};
use std::ops::Neg;

pub fn move_system(
    time: Res<Time>,
    mut move_timer: ResMut<MoveTimer>,
    mut mob_query: Query<(&mut Transform, &mut Position, &mut PathfindingSteps), With<Creature>>,
    grid: ResMut<IsoGrid>,
) {
    move_timer.0.tick(time.delta());
    if move_timer.0.tick(time.delta()).just_finished() {
        // this should be multiple mobs in the future
        // this ideally should calculate the direction to go between the actual position with the
        // next step and then move the mob in that direction
        for (mut mob_transform, mut mob_pos, mut mob_step) in mob_query.iter_mut() {
            let Some(next_step) = mob_step.steps.pop_front() else {
                return;
            };
            // TODO recalculate after path was blocked?
            // TODO check after updating the blocked_coords?
            if grid.blocked_coords.contains(&next_step) {
                mob_step.steps.clear();
                return;
            }

            let step = grid.layout.tile_to_world_pos(next_step);
            // TODO instead of changing to the block calculate the direction and move to the block in a fixed speed

            mob_transform.translation.x = step.x;
            mob_transform.translation.y = step.y;
            mob_transform.translation.z = step.y.neg() + step.z + 1.0;

            *mob_pos = next_step;
        }
    }
}
