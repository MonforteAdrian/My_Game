use crate::{Chase, Creature, IsoGrid, Position, Targets, Viewshed};
use bevy::prelude::{Entity, EventWriter, Query, Res, With};

pub fn visibility_system(
    mut query: Query<(Entity, &Position, &Viewshed), With<Creature>>,
    mut chase_entity_event: EventWriter<Chase>,
    grid: Res<IsoGrid>,
) {
    for (entity, pos, viewshed) in query.iter_mut() {
        // Better check every entity if is in the viewshed
        // OR
        // check every viewshed position if there is entity collapsing
        // Check if there is a colliding keys in hashmaps
        // TODO improve this
        for (other_pos, other_entity) in grid.entities.iter() {
            if viewshed.visible_tiles.contains(other_pos) && other_pos != pos {
                // TODO in the future this should check factions and those things
                chase_entity_event.send(Chase {
                    creator: Some(entity),
                    targets: Targets::Single { target: *other_entity },
                });
            }
        }
    }
}
