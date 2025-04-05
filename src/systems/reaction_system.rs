use crate::{Chasing, Creature, Direction, Health, CurrentMap, Position};
use bevy::prelude::{Changed, Query, With};

#[allow(clippy::type_complexity)]
pub fn reaction_system(
    mut query: Query<(Option<&Chasing>, &mut Direction, &Health), (With<Creature>, Changed<Health>)>,
) {
    // This should only be triggered when the creature moves, either to another tile or facing direction
    for (chasing, mut direction, health) in query.iter_mut() {
        if chasing.is_some() || health.current == health.max {
            continue;
        }
        // TODO improve this shitty code
        direction.0 += 4;
        direction.0 %= 8;
    }
}
