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
        app.init_resource::<IsoGrid>()
            .init_resource::<WorldMap>()
            .insert_resource(MoveTimer(Timer::from_seconds(0.1, TimerMode::Repeating)))
            .add_plugins(StatePlugin);
    }
}

// TODO there is probably a built in solution for this in bevy investigate
#[derive(Resource)]
pub struct MoveTimer(pub Timer);
