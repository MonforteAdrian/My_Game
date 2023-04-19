mod game;
mod loading;
mod menu;
mod splash;

mod prelude {
    pub(crate) use super::AppState;
    pub(crate) use crate::despawn_screen;
    pub(crate) use crate::loading::*;
    // pub(crate) use crate::utils::ConstHandles;
}

use crate::game::GamePlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::splash::SplashPlugin;

use bevy::app::App;
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Loading,
    Splash,
    Menu,
    Game,
}

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>()
            // .add_plugin(LoadingPlugin)
            .add_plugin(SplashPlugin)
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(GamePlugin);
    }
}

// Generic system that takes a component as a parameter, and will despawn all entities with that
// component
pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
