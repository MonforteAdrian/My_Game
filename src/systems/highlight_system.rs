use crate::{Creature, Position, SpawnEntity, SpawnType, Viewshed, ViewshedHighlight};
use bevy::prelude::{Changed, Commands, Entity, EventWriter, Query, With};
use std::collections::HashSet;

pub fn viewshed_highlight_system(
    mut commands: Commands,
    mut spawn_event: EventWriter<SpawnEntity>,
    mut query: Query<&Viewshed, (With<Creature>, Changed<Viewshed>)>,
    highlighted_query: Query<(Entity, &Position, &ViewshedHighlight)>,
) {
    // First clear all the viewshedhighlight that is not in the new
    if query.is_empty() {
        return;
    }

    let mut new_visible_tiles = HashSet::new();
    for viewshed in query.iter_mut() {
        // Spawn the highligh to the positions in viewshed and add the viewshedHighlight
        for tile in &viewshed.visible_tiles {
            new_visible_tiles.insert(*tile);
            spawn_event.write(SpawnEntity {
                name: "ViewshedFloor".to_string(),
                pos: SpawnType::AtPosition {
                    x: tile.x,
                    y: tile.y,
                    z: tile.z,
                },
            });
        }
    }

    for (entity, pos, _) in highlighted_query.iter() {
        if !new_visible_tiles.contains(pos) {
            commands.entity(entity).despawn();
        }
    }
}
