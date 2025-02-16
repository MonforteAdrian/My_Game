#![feature(let_chains)]
#![feature(test)]
extern crate test;

mod ai;
mod camera;
mod components;
mod effects;
mod game;
mod helpers;
mod map;
mod menus;
mod raws;
mod resources;
mod spawner;
mod splash;
mod systems;
pub use ai::*;

pub(crate) use components::*;
pub(crate) use effects::*;
pub(crate) use helpers::*;
pub(crate) use map::*;
pub(crate) use raws::*;
pub(crate) use resources::*;
pub(crate) use spawner::*;

use camera::CameraPlugin;
use effects::EffectsPlugin;
use game::GamePlugin;
use map::WorldCreationPlugin;
use menus::MenuPlugin;
use raws::RawsPlugin;
use resources::ResourcesPlugin;
use spawner::SpawnerPlugin;
use splash::SplashPlugin;
use systems::SystemsPlugin;

use bevy::{app::App, prelude::*};

#[cfg(debug_assertions)]
use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    text::FontSmoothing,
};

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SplashPlugin)
            .add_plugins(ResourcesPlugin)
            .add_plugins(RawsPlugin)
            .add_plugins(CameraPlugin)
            .add_plugins(MenuPlugin)
            .add_plugins(SystemsPlugin)
            .add_plugins(SpawnerPlugin)
            .add_plugins(EffectsPlugin)
            .add_plugins(WorldCreationPlugin)
            .add_plugins(GamePlugin)
            // Reflect
            .register_type::<Position>()
            .register_type::<Viewshed>()
            .register_type::<PathfindingSteps>()
            .register_type::<Direction>()
            .register_type::<Health>();

        #[cfg(debug_assertions)]
        {
            //use bevy_inspector_egui::quick::WorldInspectorPlugin;

            app.add_plugins(FpsOverlayPlugin {
                config: FpsOverlayConfig {
                    text_config: TextFont {
                        // Here we define size of our overlay
                        font_size: 16.0,
                        // If we want, we can use a custom font
                        font: default(),
                        // We could also disable font smoothing,
                        font_smoothing: FontSmoothing::default(),
                    },
                    // We can also change color of the overlay
                    text_color: Color::srgb(0.0, 1.0, 0.0),
                    enabled: true,
                },
            });
            // This hurts the performance hugely so be mindful on usage
            //.add_plugins(WorldInspectorPlugin::new())
        }
    }
}

// TODO delete this in favor of 0.14 scoped screens check game_menu example
// OR
// TODO move this to helpers utils
// Generic system that takes a component as a parameter, and will despawn all entities with that
// component
pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
