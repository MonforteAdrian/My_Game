use crate::{Creature, SpawnEntity, SpawnType, Viewshed, ViewshedHighlight};
use bevy::prelude::{Changed, Commands, Entity, EventWriter, Query, With};

pub fn viewshed_highlight_system(
    mut commands: Commands,
    mut spawn_event: EventWriter<SpawnEntity>,
    mut query: Query<&Viewshed, (With<Creature>, Changed<Viewshed>)>,
    highlighted_query: Query<(Entity, &ViewshedHighlight)>,
) {
    // First clear all the viewshedhighlight that is not in the new
    if !query.is_empty() {
        for (entity, _) in &highlighted_query {
            commands.entity(entity).despawn();
        }
    }
    for viewshed in query.iter_mut() {
        // Spawn the highligh to the positions in viewshed and add the viewshedHighlight
        for tile in &viewshed.visible_tiles {
            spawn_event.send(SpawnEntity {
                name: "ViewshedFloor".to_string(),
                pos: SpawnType::AtPosition { x: tile.x, y: tile.y, z: tile.z },
            });
        }
    }
}