use bevy::prelude::*;

mod map;
pub use map::*;
mod states;
pub use states::*;
mod world_map;
pub use world_map::*;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentMap>()
            .init_resource::<WorldMap>()
            // configure our fixed timestep schedule to run twenty times per second
            .insert_resource(Time::<Fixed>::from_seconds(0.05))
            .add_plugins(StatePlugin);
    }
}
