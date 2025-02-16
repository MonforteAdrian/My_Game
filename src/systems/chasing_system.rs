use crate::{Chasing, Creature, MoveTo, PathfindingSteps, Position, Targets};
use bevy::prelude::{Commands, Entity, EventWriter, Query, With};

pub fn chasing_system(
    mut commands: Commands,
    mut move_entity_to_event: EventWriter<MoveTo>,
    mut chaser_query: Query<(Entity, &Chasing, &PathfindingSteps), With<Chasing>>,
    creatures_query: Query<&Position, With<Creature>>,
) {
    for (chaser_entity, chasing, steps) in chaser_query.iter_mut() {
        // Get the target position if the target cannot be found remove the Chasing
        let Ok(target_pos) = creatures_query.get(chasing.0) else {
            commands.entity(chaser_entity).remove::<Chasing>();
            continue;
        };
        // Get the previous target position if empty remove Chasing
        // TODO this might cause problems when this runs but the path was not yet calculated
        let Some(previous_target_pos) = steps.back() else {
            commands.entity(chaser_entity).remove::<Chasing>();
            continue;
        };
        // If the target didn't move simply continue
        if previous_target_pos == target_pos {
            continue;
        }
        // If the target moved recalculate the pathfinding
        move_entity_to_event.send(MoveTo {
            creator: Some(chaser_entity),
            targets: Targets::Tile { tile: *target_pos },
        });
    }
}
