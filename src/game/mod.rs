use bevy::prelude::*;
use super::{despawn_screen, GameState, TEXT_COLOR};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(setup))
            .add_system_set(SystemSet::on_update(GameState::Game).with_system(game))
            .add_system_set(SystemSet::on_exit(GameState::Game).with_system(despawn_screen::<OnGameScreen>));
    }
}

#[derive(Component)]
struct OnGameScreen;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                margin: UiRect::all(Val::Auto),
                ..default()
            },
            color: Color::BLACK.into(),
            ..default()
        })
        .insert(OnGameScreen)
        .with_children(|parent| {
            parent.spawn_bundle(
                TextBundle::from_section(
                    "Hola, puedes apretar escape para volver al menu",
                    TextStyle {
                        font: font.clone(),
                        font_size: 50.0,
                        color: TEXT_COLOR,
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(50.0)),
                    ..default()
                }),
            );
        });
}

fn game(
    keyboard_input: Res<Input<KeyCode>>,
    mut game_state: ResMut<State<GameState>>,
) {
    if keyboard_input.pressed(KeyCode::Escape){
        game_state.set(GameState::Menu).unwrap();
    }
}
