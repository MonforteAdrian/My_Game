use crate::prelude::*;
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), game_setup)
            .add_systems(
                Update,
                (
                    game.run_if(in_state(GameState::InGame)),
                    toggle_pause.run_if(in_state(GameState::InGame)),
                ),
            )
            .add_systems(OnExit(GameState::InGame), despawn_screen::<OnGameScreen>);
    }
}

#[derive(Component)]
struct OnGameScreen;

fn game_setup() {}

fn game(keyboard_input: Res<ButtonInput<KeyCode>>, mut game_state: ResMut<NextState<GameState>>) {
    if keyboard_input.pressed(KeyCode::Escape) {
        game_state.set(GameState::InMenu);
    }
}

fn toggle_pause(
    input: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<IsPaused>>,
    mut next_state: ResMut<NextState<IsPaused>>,
) {
    if input.just_pressed(KeyCode::Space) {
        next_state.set(match current_state.get() {
            IsPaused::Running => IsPaused::Paused,
            IsPaused::Paused => IsPaused::Running,
        });
    }
}
