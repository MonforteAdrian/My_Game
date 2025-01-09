use bevy::prelude::*;

mod map;
pub use map::*;
mod states;
pub use states::*;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<IsoGrid>()
            .insert_resource(MoveTimer(Timer::from_seconds(0.2, TimerMode::Repeating)))
            .add_plugins(StatePlugin);
    }
}

// TODO there is probably a built in solution for this in bevy investigate
#[derive(Resource)]
pub struct MoveTimer(pub Timer);
