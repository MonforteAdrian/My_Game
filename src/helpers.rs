mod camera;
mod controls;
mod state;

use bevy::prelude::*;

use camera::CameraPlugin;
use controls::ControlsPlugin;
use state::StatePlugin;

pub use state::GameState;
pub use state::IsPaused;
pub use state::WorldCreationState;
pub use state::MenuState;

pub struct HelpersPlugin;

impl Plugin for HelpersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CameraPlugin, ControlsPlugin, StatePlugin));
    }
}
