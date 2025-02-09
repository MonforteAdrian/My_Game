use super::*;

/// Simulates game state that can affect AI decision-making
#[derive(Component)]
pub struct WorldState {
    pub resources_available: bool,
    pub enemies_nearby: bool,
}

impl WorldState {
    pub fn is_action_valid(&self, action: PrimitiveTask) -> bool {
        match action {
            PrimitiveTask::GatherResources => self.resources_available,
            PrimitiveTask::AttackEnemy => self.enemies_nearby,
            _ => true,
        }
    }
}
