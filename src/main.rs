mod game;
mod menu;

use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "My Game".to_string(),
            ..Default::default()
        })
        .add_state(GameState::Menu)
        .add_plugins(DefaultPlugins)
        .add_plugin(game::GamePlugin)
        .add_plugin(menu::MenuPlugin)
        .run();
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Game,
    Menu,
}
