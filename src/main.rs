use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use my_game::AppPlugin;

fn main() {
    App::new()
        // Hot reloading assets
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "My Game".into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(DefaultPickingPlugins)
        .add_plugins(AppPlugin)
        .run();
}
