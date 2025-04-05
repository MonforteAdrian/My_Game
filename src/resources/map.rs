use crate::Position;
use crate::map::Layout;
use bevy::prelude::{Entity, Resource};
use std::collections::{HashMap, HashSet};

// TODO convert this to resources in map creation CurrentWorld
#[derive(Default, Debug, Resource)]
pub struct CurrentMap {
    // This can probably be changed to a HashSet<Entity> as each entity has a Position component
    pub tiles: HashMap<Position, Entity>,
    pub entities: HashMap<Position, Entity>,
    pub items: HashMap<Position, Entity>,
    pub layout: Layout,
    pub blocked_coords: HashSet<Position>,
}
