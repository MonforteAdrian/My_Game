use bevy::{asset::ChangeWatcher, prelude::*, utils::Duration};
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
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(200)),
                    ..default()
                }),
        )
        .add_plugins(AppPlugin)
        .run();
}
