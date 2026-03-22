use bevy::prelude::ReflectResource;
use bevy::app::{App, Plugin, Startup, Update};
use bevy::asset::{Assets, Handle};
use bevy::color::Color;
use bevy::log::warn;
use bevy::math::Vec3;
use bevy::mesh::{Mesh, Mesh3d, Meshable};
use bevy::pbr::{MeshMaterial3d, StandardMaterial};
use bevy::prelude::{Added, Click, Commands, Component, Entity, EntityEvent, On, Out, Over, Pointer, PointerButton, Query, Res, ResMut, Resource, Sphere, Transform, Visibility, With};
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
    visible_material: Handle<StandardMaterial>,
    hovered_material: Handle<StandardMaterial>,
}

#[derive(Component, Reflect)]
#[require(Transform, Visibility)]
pub struct Clickable(pub f32);

fn init_clickables(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    let sphere_mesh = meshes.add(Sphere::new(1.0).mesh());
    let visible_material = materials.add(StandardMaterial::from(Color::linear_rgba(0.1, 0.4, 0.1, 0.1)));
    let hovered_material = materials.add(StandardMaterial::from(Color::linear_rgba(0.1, 0.6, 0.1, 0.2)));
    commands.insert_resource(ClickableParams { sphere_mesh, visible_material, hovered_material });
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
                MeshMaterial3d(params.visible_material.clone()),
            ))
            .observe(update_material_on::<Pointer<Over>>(params.hovered_material.clone()))
            .observe(update_material_on::<Pointer<Out>>(params.visible_material.clone()));
    }
}

fn update_material_on<E: EntityEvent>(
    new_material: Handle<StandardMaterial>,
) -> impl Fn(On<E>, Query<&mut MeshMaterial3d<StandardMaterial>>) {
    move |event, mut query| {
        if let Ok(mut material) = query.get_mut(event.event_target()) {
            material.0 = new_material.clone();
        }
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
