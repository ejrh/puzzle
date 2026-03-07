pub mod debug;
pub mod camera_move;
pub mod named_scene;

use bevy::prelude::*;

pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(debug::DebugPlugin)
            .add_systems(Update, camera_move::camera_move)
            .add_systems(Update, named_scene::update_named_scenes);
    }
}
