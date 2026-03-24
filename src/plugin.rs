use bevy::app::{App, Plugin, Startup};
use bevy::asset::AssetServer;
use bevy::camera::Camera3d;
use bevy::camera_controller::free_camera::{FreeCamera, FreeCameraPlugin};
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::DefaultPlugins;
use bevy::ecs::system::{Res, Commands};
use bevy::light::PointLight;
use bevy::math::{EulerRot, Quat, Vec3};
use bevy::prelude::{ChildOf, MeshPickingPlugin, Name, Transform};

use crate::clickable::ClickablePlugin;
use crate::item::ItemPlugin;
use crate::logic::LogicPlugin;
use crate::puzzle::{Puzzle, PuzzlePlugin};
use crate::utils::UtilsPlugin;

pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(DefaultPlugins)
            .add_plugins(MeshPickingPlugin)
            .add_plugins(FreeCameraPlugin);
        app
            .add_plugins(UtilsPlugin);

        app.add_systems(Startup, setup_camera);
        app.add_systems(Startup, setup_lights);
        app.add_systems(Startup, setup_puzzle);

        app
            .add_plugins(ClickablePlugin)
            .add_plugins(ItemPlugin)
            .add_plugins(LogicPlugin)
            .add_plugins(PuzzlePlugin);
    }
}

fn setup_camera(mut commands: Commands) {
    let camera_id = commands.spawn((
        Camera3d::default(),
        Tonemapping::None,
        Transform::from_xyz(0.0, 2.0, 4.0).looking_at(Vec3::new(0.0, 1.0, -4.0), Vec3::Y),
        FreeCamera::default(),
        Name::new("camera"),
    )).id();

    commands.spawn((
        Transform::from_xyz(0.6, 0.0, -1.3).with_rotation(Quat::from_euler(EulerRot::XYZ, 5.6, 0.2, 3.4)),
        Name::new("hand"),
        ChildOf(camera_id),
    ));
}

fn setup_lights(mut commands: Commands) {
    commands.spawn((
        PointLight::default(),
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}

fn setup_puzzle(
    asset_server: Res<AssetServer>,
    mut commands: Commands
) {
    const PUZZLE_PATH: &str = "puzzle.ron";

    let puzzle_def = asset_server.load(PUZZLE_PATH);

    commands.spawn((
        Puzzle(puzzle_def),
        Name::new("Puzzle"),
    ));
}
