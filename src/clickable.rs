use bevy::prelude::ReflectResource;
use bevy::app::{App, Plugin, Startup, Update};
use bevy::asset::{Assets, Handle};
use bevy::log::warn;
use bevy::math::Vec3;
use bevy::mesh::{Mesh, Mesh3d, Meshable};
use bevy::prelude::{Added, Click, Commands, Component, Entity, On, Pointer, PointerButton, Query, Res, ResMut, Resource, Sphere, Transform, Visibility, With};
use bevy::reflect::Reflect;

use crate::logic::LogicMessage;

pub struct ClickablePlugin;

impl Plugin for ClickablePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_clickables);
        app.add_systems(Update, add_clickables);

        app.add_observer(on_click);
    }
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct ClickableParams {
    sphere_mesh: Handle<Mesh>,
}

#[derive(Component, Reflect)]
#[require(Transform, Visibility)]
pub struct Clickable(pub f32);

fn init_clickables(
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
) {
    let sphere_mesh = meshes.add(Sphere::new(1.0).mesh());
    commands.insert_resource(ClickableParams { sphere_mesh });
}

fn add_clickables(
    params: Res<ClickableParams>,
    clickables: Query<(Entity, &Clickable, &mut Transform), Added<Clickable>>,
    mut commands: Commands,
) {
    for (id, clickable, mut transform) in clickables {
        transform.scale = Vec3::splat(clickable.0);
        commands.entity(id)
            .insert((
                Mesh3d(params.sphere_mesh.clone()),
            ));
    }
}

fn on_click(
    event: On<Pointer<Click>>,
    clickables: Query<(), With<Clickable>>,
    mut command: Commands,
) {
    let target = event.entity;
    let primary = matches!(event.event.button, PointerButton::Primary);

    if !clickables.contains(target) {
        warn!("not clickable: {}", target);
    }

    command.write_message(LogicMessage::Clicked(target, primary));
}
