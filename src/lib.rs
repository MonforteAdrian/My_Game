mod components;
mod game;
mod helpers;
mod map;
mod menus;
mod splash;

mod prelude {
    pub(crate) use crate::despawn_screen;
    pub(crate) use crate::helpers::GameState;
    pub(crate) use crate::helpers::IsPaused;
    pub(crate) use crate::helpers::MenuState;
    pub(crate) use crate::helpers::WorldCreationState;
}

use crate::game::GamePlugin;
use crate::helpers::HelpersPlugin;
use crate::map::WorldCreationPlugin;
use crate::menus::MenuPlugin;
use crate::splash::SplashPlugin;

#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::{app::App, prelude::*};

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SplashPlugin)
            .add_plugins(HelpersPlugin)
            .add_plugins(MenuPlugin)
            .add_plugins(WorldCreationPlugin)
            .add_plugins(GamePlugin);

        #[cfg(debug_assertions)]
        {
            app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
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
