use crate::map::Layout;
use crate::Position;
use bevy::{
    prelude::{Entity, Resource},
    utils::{HashMap, HashSet},
};

// TODO convert this to resources in map creation CurrentWorld
#[derive(Default, Debug, Resource)]
pub struct IsoGrid {
    // This can probably be changed to a HashSet<Entity> as each entity has a Position component
    pub entities: HashMap<Position, Entity>,
    pub layout: Layout,
    pub blocked_coords: HashSet<Position>,
}
