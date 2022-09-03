use crate::GameState;
use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
//        app.add_system_set(SystemSet::on_enter(GameState::Menu).with_system(setup))
//            .add_system_set(SystemSet::on_exit(GameState::Menu).with_system(exit))
//            .add_system_set(SystemSet::on_update(GameState::Menu).with_system(input_system));
    }
}
