mod game;
mod helpers;
mod loading;
mod menu;
mod splash;
mod world_creation;

mod prelude {
    pub(crate) use crate::despawn_screen;
    pub(crate) use crate::helpers::GameState;
    pub(crate) use crate::helpers::IsPaused;
    pub(crate) use crate::helpers::WorldCreationState;
    pub(crate) use crate::helpers::MenuState;
    pub(crate) use crate::loading::*;
}

use crate::game::GamePlugin;
use crate::helpers::HelpersPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::splash::SplashPlugin;
use crate::world_creation::WorldCreationPlugin;

use bevy::{app::App, prelude::*};

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LoadingPlugin)
            .add_plugins(SplashPlugin)
            .add_plugins(HelpersPlugin)
            .add_plugins(MenuPlugin)
            .add_plugins(WorldCreationPlugin)
            .add_plugins(GamePlugin);
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
