mod game;
mod menu;
mod splash;

use bevy::prelude::*;


fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "My Game".to_string(),
            ..Default::default()
        })
        .add_startup_system(setup)
        .add_state(GameState::Splash)
        .add_plugins(DefaultPlugins)
        .add_plugin(game::GamePlugin)
        .add_plugin(menu::MenuPlugin)
        .add_plugin(splash::SplashPlugin)
        .run();
}

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Game,
    Menu,
    Splash,
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

// Generic system that takes a component as a parameter, and will despawn all entities with that
// component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
