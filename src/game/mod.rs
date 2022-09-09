mod camera;
mod tilemap;

use bevy::prelude::*;
use super::{despawn_screen, AppState};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state(GameState::Disabled)
            .add_plugin(camera::CameraPlugin)
            .add_plugin(tilemap::TilemapPlugin)
            .add_system_set(SystemSet::on_enter(AppState::Game).with_system(setup))
            .add_system_set(SystemSet::on_exit(AppState::Game).with_system(despawn_screen::<OnGameScreen>))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(game));
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    MapCreation,
    Gameplay,
    Disabled,
}

#[derive(Component)]
struct OnGameScreen;

fn setup(mut game_state: ResMut<State<GameState>>) {
    let _ = game_state.set(GameState::MapCreation);
}

fn game(
    keyboard_input: Res<Input<KeyCode>>,
    mut game_state: ResMut<State<AppState>>,
) {
    if keyboard_input.pressed(KeyCode::Escape){
        game_state.set(AppState::Menu).unwrap();
    }
}
