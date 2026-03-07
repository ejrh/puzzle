use bevy::app::{App, Plugin, Update};
use bevy::camera::Camera;
use bevy::log::info;
use bevy::prelude::{Added, Click, Commands, Component, Entity, EntityEvent, Name, On, Pointer, PointerButton, Query, Reflect, ResMut, Resource, Single, Transform, With};

use crate::utils::camera_move::CameraMovingTo;

pub struct ZonePlugin;

impl Plugin for ZonePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentZone>();
        app.add_systems(Update, add_zones);
        app.add_observer(on_unzoom);
    }
}

#[derive(Default, Resource)]
pub struct CurrentZone(Option<Entity>);

#[derive(Component, Reflect)]
pub struct Zone;

#[derive(Component, Reflect)]
pub struct ActiveInZones(pub Vec<Entity>);

#[derive(Component, Reflect)]
pub struct ZoneCamera(pub Transform);

#[derive(Component, Reflect)]
pub struct UnzoomTo(pub Entity);

fn add_zones(
    added_zones: Query<Entity, Added<Zone>>,
    mut commands: Commands,
) {
    for id in added_zones {
        commands.entity(id).observe(on_zoom);
    }
}

fn on_zoom(
    event: On<Pointer<Click>>,
    zones: Query<(&ZoneCamera, &Name)>,
    mut current: ResMut<CurrentZone>,
    camera_id: Single<Entity, With<Camera>>,
    mut command: Commands,
) {
    if !matches!(event.event.button, PointerButton::Primary) { return }

    let Ok((zone_camera, name)) = zones.get(event.event_target())
    else { return; };

    info!("Clicked zone: {name}");

    current.0 = Some(event.event_target());

    command.entity(*camera_id).insert(CameraMovingTo(zone_camera.0, 2.0));
}

fn on_unzoom(
    event: On<Pointer<Click>>,
    unzoomables: Query<(&UnzoomTo, &Name)>,
    zones: Query<(&Zone, &ZoneCamera, &Name)>,
    mut current: ResMut<CurrentZone>,
    camera_id: Single<Entity, With<Camera>>,
    mut command: Commands,
) {
    if !matches!(event.event.button, PointerButton::Secondary) { return }

    let Some(current_id) = current.0
    else { return };
    let Ok((unzoom_to, from_name)) = unzoomables.get(current_id)
    else { return; };

    let Ok((_, zoomable_camera, to_name)) = zones.get(unzoom_to.0)
    else { return; };

    info!("Clicked unzoom: {from_name} back to {to_name}");

    current.0 = Some(unzoom_to.0);

    command.entity(*camera_id).insert(CameraMovingTo(zoomable_camera.0, 1.0));
}
