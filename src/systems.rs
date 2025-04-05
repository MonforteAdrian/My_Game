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
mod field_of_view_system;
use field_of_view_system::*;
mod chasing_system;
use chasing_system::*;
mod reaction_system;
use reaction_system::*;
mod position_check_system;
use position_check_system::*;

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                chasing_system,
                field_of_view_system,
                visibility_system,
                viewshed_highlight_system,
                reaction_system,
                position_check_system,
            )
                .chain()
                .run_if(in_state(GameState::InGame)),
        )
        .add_systems(FixedUpdate, move_system.run_if(in_state(GameState::InGame)));
    }
}
