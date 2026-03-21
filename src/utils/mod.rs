pub mod debug;
pub mod movement;
pub mod named_scene;

use bevy::prelude::*;

pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(debug::DebugPlugin)
            .add_systems(Update, movement::update_movement)
            .add_systems(Update, named_scene::update_named_scenes);
    }
}
