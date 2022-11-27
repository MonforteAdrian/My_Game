mod game;
mod menu;
mod splash;

use bevy::{app::PluginGroupBuilder, prelude::*};


fn main() {
    App::new()
        // Background Color
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        // Hot reloading assets
        .add_state(AppState::Splash)
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                window: WindowDescriptor {
                    title: "My Game".to_string(),
                    ..default()
                },
                ..default()
            })
            .set(AssetPlugin {
                watch_for_changes: true,
                ..default()
            })
        )
        .add_plugins(MyPlugins)
        .run();
}

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum AppState {
    Game,
    Menu,
    Splash,
}

// Generic system that takes a component as a parameter, and will despawn all entities with that
// component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

pub struct MyPlugins;

impl PluginGroup for MyPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(game::GamePlugin)
            .add(menu::MenuPlugin)
            .add(splash::SplashPlugin)
    }
}
