use bevy::{
    prelude::{default, App, DefaultPlugins, ImagePlugin, PluginGroup, Window, WindowPlugin},
    //window::PresentMode,
};
use my_game::AppPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "My Game".into(),
                        // FPS if you want to change the fps to more than 60 this is your place
                        //present_mode: PresentMode::Immediate,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            AppPlugin,
        ))
        .run();
}
