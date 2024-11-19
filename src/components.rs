use crate::map::Position;
use bevy::prelude::Component;
use std::collections::VecDeque;

#[derive(Component)]
pub struct Renderable {
    pub sprite: String, //TODO now is the path. IMPROVE this
}

#[derive(Component)]
pub struct Mob {}

#[derive(Component)]
pub struct PathfindingSteps {
    pub steps: VecDeque<Position>,
}
