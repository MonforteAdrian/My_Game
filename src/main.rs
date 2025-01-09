use bevy::prelude::{default, App, DefaultPlugins, ImagePlugin, PluginGroup, Window, WindowPlugin};
use my_game::AppPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window { title: "My Game".into(), ..default() }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            AppPlugin,
        ))
        .run();
}
