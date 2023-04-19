mod camera;
mod map;

use super::{despawn_screen, AppState};
use crate::prelude::*;
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_plugin(camera::CameraPlugin)
            .add_plugin(map::MapPlugin)
            .add_system(setup.in_schedule(OnEnter(AppState::Game)))
            .add_system(despawn_screen::<OnGameScreen>.in_schedule(OnExit(AppState::Game)))
            .add_system(game.in_set(OnUpdate(AppState::Game)));
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Disabled,
    MapCreation,
    Gameplay,
}

#[derive(Component)]
struct OnGameScreen;

fn setup(mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::MapCreation);
}

fn game(keyboard_input: Res<Input<KeyCode>>, mut game_state: ResMut<NextState<AppState>>) {
    if keyboard_input.pressed(KeyCode::Escape) {
        game_state.set(AppState::Menu);
    }
}
