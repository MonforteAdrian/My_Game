mod camera;
mod controls;

use bevy::prelude::*;

use camera::CameraPlugin;
use controls::ControlsPlugin;

pub struct HelpersPlugin;

impl Plugin for HelpersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CameraPlugin).add_plugins(ControlsPlugin);
    }
}
