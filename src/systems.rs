use crate::GameState;
use bevy::prelude::*;

mod ai_system;
use ai_system::*;
mod movement_system;
use movement_system::*;
mod highlight_system;
use highlight_system::*;
mod visibility_system;
use visibility_system::*;

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (visibility_system, viewshed_highlight_system).run_if(in_state(GameState::InGame)),
        )
        .add_systems(FixedUpdate, move_system.run_if(in_state(GameState::InGame)));
    }
}
