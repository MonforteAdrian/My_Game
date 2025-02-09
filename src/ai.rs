use bevy::prelude::Component;

mod world_state;
pub use world_state::*;
mod domain;
pub use domain::*;
mod tasks;
pub use tasks::*;

/// Bevy component for AI-controlled entities
#[derive(Component)]
pub struct AI {
    pub domain: Domain,
    pub world_state: WorldState,
}
