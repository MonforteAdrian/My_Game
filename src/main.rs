use::bevy::prelude::*;
use my_plugin::MyPlugin;
use my_plugin::resources::BoardOptions;

#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;

fn camera_setup(mut commands: Commands) {
    // By default camera Z is 999.9 so max z visible is 999
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn main() {
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        title: "Mi juego!".to_string(),
        ..Default::default()
    })
    .add_plugins(DefaultPlugins);
    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new());
    app.insert_resource(BoardOptions {
        map_size: (20, 20),
        ..Default::default()
    })
    .add_plugin(MyPlugin);
    app.add_startup_system(camera_setup);
    app.run();
}
