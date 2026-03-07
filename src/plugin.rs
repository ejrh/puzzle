use bevy::app::{App, Plugin, Startup, Update};
use bevy::asset::AssetServer;
use bevy::camera::Camera3d;
use bevy::camera_controller::free_camera::{FreeCamera, FreeCameraPlugin};
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::DefaultPlugins;
use bevy::ecs::system::{Res, Commands};
use bevy::light::PointLight;
use bevy::math::{Dir3, Vec3};
use bevy::prelude::{Component, MeshPickingPlugin, Name, Query, Transform, With};
use bevy::time::Time;

use crate::clickable::ClickablePlugin;
use crate::puzzle::{Puzzle, PuzzlePlugin};
use crate::utils::{UtilsPlugin, named_scene::NamedScene};
use crate::zone::ZonePlugin;

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

        app.add_systems(Update, animate_stuff);

        app
            .add_plugins(ClickablePlugin)
            .add_plugins(PuzzlePlugin)
            .add_plugins(ZonePlugin);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Tonemapping::None,
        Transform::from_xyz(0.0, 2.0, -4.0).looking_at(Vec3::new(0.0, 1.0, 4.0), Vec3::Y),
        FreeCamera::default(),
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
    const GLTF_PATH: &str = "puzzle.gltf";

    let gltf = asset_server.load(GLTF_PATH);

    commands.spawn((
        NamedScene { gltf: gltf.clone(), scene_name: "Chest".to_string() },
        Transform::from_xyz(2.0, 0.0, 4.0),
    ));
    commands.spawn((
        NamedScene { gltf: gltf.clone(), scene_name: "Lid".to_string() },
        Transform::from_xyz(2.0, 0.0, 4.0),
    ));
    commands.spawn((
        NamedScene { gltf: gltf.clone(), scene_name: "Key".to_string() },
        Transform::from_xyz(-2.0, 1.5, 4.0),
        Rotating,
    ));

    const PUZZLE_PATH: &str = "puzzle.ron";

    let puzzle_def = asset_server.load(PUZZLE_PATH);

    commands.spawn((
        Puzzle(puzzle_def),
        Name::new("Puzzle"),
    ));
}

#[derive(Component)]
struct Rotating;

fn animate_stuff(
    stuff: Query<&mut Transform, With<Rotating>>,
    time: Res<Time>,
) {
    let rate_per_second = 45_f32.to_radians();
    for mut thing in stuff {
        thing.rotate_axis(Dir3::Y, rate_per_second * time.delta().as_secs_f32());
    }
}
