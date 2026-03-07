use bevy::app::{App, Plugin, Startup, Update};
use bevy::asset::{Assets, Handle};
use bevy::camera::Camera;
use bevy::color::Color;
use bevy::ecs::observer::ObservedBy;
use bevy::log::info;
use bevy::math::Vec3;
use bevy::mesh::{Mesh, Mesh3d, Meshable};
use bevy::pbr::{MeshMaterial3d, StandardMaterial};

use bevy::prelude::{Changed, Click, Commands, Component, Entity, EntityEvent, Name, On, Out, Over, Pointer, PointerButton, Query, Res, ResMut, Resource, Single, Sphere, Transform, With};

use crate::utils::camera_move::CameraMovingTo;

pub struct ZoomablePlugin;

impl Plugin for ZoomablePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentZoomable>();
        app.add_systems(Startup, init_zoomables);
        app.add_systems(Update, update_zoomables);
        app.add_observer(on_unzoom);
    }
}

#[derive(Default, Resource)]
pub struct CurrentZoomable(Option<Entity>);

#[derive(Component)]
pub struct Zoomable(pub f32);

#[derive(Component)]
pub struct ZoomableCamera(pub Transform);

#[derive(Component)]
pub struct UnzoomTo(pub Entity);

#[derive(Resource)]
pub struct ZoomableParams {
    sphere_mesh: Handle<Mesh>,
    visible_material: Handle<StandardMaterial>,
    hovered_material: Handle<StandardMaterial>,
}

fn init_zoomables(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    let sphere_mesh = meshes.add(Sphere::new(1.0).mesh());
    let visible_material = materials.add(StandardMaterial::from(Color::linear_rgba(0.1, 0.4, 0.1, 0.1)));
    let hovered_material = materials.add(StandardMaterial::from(Color::linear_rgba(0.1, 0.6, 0.1, 0.2)));
    commands.insert_resource(ZoomableParams { sphere_mesh, visible_material, hovered_material });
}

fn update_zoomables(
    params: Res<ZoomableParams>,
    zoomables: Query<(Entity, &Zoomable, &mut Transform), Changed<Zoomable>>,
    mut commands: Commands,
) {
    for (id, zoomable, mut transform) in zoomables {
        transform.scale = Vec3::splat(zoomable.0);
        commands.entity(id).remove::<ObservedBy>();
        commands.entity(id).insert((
            Mesh3d(params.sphere_mesh.clone()),
            MeshMaterial3d(params.visible_material.clone()),
        )).observe(update_material_on::<Pointer<Over>>(params.hovered_material.clone()))
        .observe(update_material_on::<Pointer<Out>>(params.visible_material.clone()))
        .observe(on_zoom);
    }
}

fn update_material_on<E: EntityEvent>(
    new_material: Handle<StandardMaterial>,
) -> impl Fn(On<E>, Query<&mut MeshMaterial3d<StandardMaterial>>) {
    // An observer closure that captures `new_material`. We do this to avoid needing to write four
    // versions of this observer, each triggered by a different event and with a different hardcoded
    // material. Instead, the event type is a generic, and the material is passed in.
    move |event, mut query| {
        if let Ok(mut material) = query.get_mut(event.event_target()) {
            material.0 = new_material.clone();
        }
    }
}

fn on_zoom(
    event: On<Pointer<Click>>,
    zoomables: Query<(&ZoomableCamera, &Name)>,
    mut current: ResMut<CurrentZoomable>,
    camera_id: Single<Entity, With<Camera>>,
    mut command: Commands,
) {
    if !matches!(event.event.button, PointerButton::Primary) { return }

    let Ok((zoomable_camera, name)) = zoomables.get(event.event_target())
    else { return; };

    info!("Clicked zoom: {name}");

    current.0 = Some(event.event_target());

    command.entity(*camera_id).insert(CameraMovingTo(zoomable_camera.0, 2.0));
}

fn on_unzoom(
    event: On<Pointer<Click>>,
    zoomables: Query<(&Zoomable, &ZoomableCamera, &Name)>,
    unzoomables: Query<(&UnzoomTo, &Name)>,
    mut current: ResMut<CurrentZoomable>,
    camera_id: Single<Entity, With<Camera>>,
    mut command: Commands,
) {
    if !matches!(event.event.button, PointerButton::Secondary) { return }

    let Some(current_id) = current.0
        else { return };
    let Ok((unzoom_to, name)) = unzoomables.get(current_id)
        else { return; };

    let Ok((_, zoomable_camera, name2)) = zoomables.get(unzoom_to.0)
    else { return; };

    info!("Clicked unzoom: {name} back to {name2}");

    current.0 = Some(unzoom_to.0);

    command.entity(*camera_id).insert(CameraMovingTo(zoomable_camera.0, 1.0));
}
