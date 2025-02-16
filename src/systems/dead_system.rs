use crate::Health;
use bevy::prelude::{info, Commands, Entity, Name, Query};

pub fn dead_system(mut commands: Commands, query: Query<(Entity, &Name, &Health)>) {
    for (entity, name, hp) in query.iter() {
        if hp.current < 1 {
            info!("{}, Just died! Rest in pieces.", name);
            commands.entity(entity).despawn();
        }
    }
}
