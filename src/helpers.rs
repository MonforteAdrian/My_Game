mod controls;
pub use controls::*;

use bevy::prelude::*;

use controls::ControlsPlugin;

//TODO remove this mod and see what to do with the controls
pub struct HelpersPlugin;

impl Plugin for HelpersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ControlsPlugin);
    }
}
