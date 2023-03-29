mod game;
mod menu;
mod splash;

use bevy::{app::PluginGroupBuilder, prelude::*};


fn main() {
    App::new()
        // Hot reloading assets
        .add_state::<AppState>()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "My Game".into(),
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest())
            .set(AssetPlugin {
                watch_for_changes: true,
                ..default()
            })
        )
        .add_plugins(MyPlugins)
        .run();
}

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Splash,
    Menu,
    Game,
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
