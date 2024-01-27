use super::{despawn_screen, AppState};
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_systems(OnEnter(AppState::Game), setup)
            .add_systems(OnExit(AppState::Game), despawn_screen::<OnGameScreen>)
            .add_systems(Update, game.run_if(in_state(AppState::Game)));
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Disabled,
    Pause,
    Gameplay,
}

#[derive(Component)]
struct OnGameScreen;

fn setup(mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::Pause);
    game_state.set(GameState::Gameplay);
}

fn game(keyboard_input: Res<Input<KeyCode>>, mut game_state: ResMut<NextState<AppState>>) {
    if keyboard_input.pressed(KeyCode::Escape) {
        game_state.set(AppState::Menu);
    }
}
