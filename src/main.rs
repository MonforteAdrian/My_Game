use::bevy::prelude::*;
use::bevy::log;
use my_plugin::MyPlugin;
use my_plugin::resources::BoardOptions;

#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    InGame,
    Paused,
    Out,
}

fn camera_setup(mut commands: Commands) {
    // By default camera Z is 999.9 so max z visible is 999
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn state_handler(mut state: ResMut<State<AppState>>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::C) {
        log::debug!("clearing detected");
        if state.current() == &AppState::InGame {
            log::info!("clearing game");
            state.set(AppState::Out).unwrap();
        }
    }
    if keys.just_pressed(KeyCode::G) {
        log::debug!("loading detected");
        if state.current() == &AppState::Out {
            log::info!("loading game");
            state.set(AppState::InGame).unwrap();
        }
    }
    if keys.just_pressed(KeyCode::Escape) {
        log::debug!("pause detected");
        if state.current() == &AppState::Paused {
            log::info!("resuming game");
            state.set(AppState::InGame).unwrap();
        }
        else {
            log::info!("pausing game");
            state.set(AppState::Paused).unwrap();
        }
    }
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
    .add_state(AppState::InGame)
    .add_system(state_handler)
    .add_plugin(MyPlugin{
        running_state: AppState::InGame,
    });
    app.add_startup_system(camera_setup);
    app.run();
}
