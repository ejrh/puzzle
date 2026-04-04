use std::f32::consts::TAU;

use bevy::prelude::{Drag, DragEnd, GlobalTransform, Name, PointerState, ReflectResource, Without};
use bevy::app::{App, Plugin, Startup, Update};
use bevy::asset::{Assets, Handle};
use bevy::camera::Camera;
use bevy::log::info;
use bevy::math::Dir3;
use bevy::mesh::{Mesh, Meshable};
use bevy::prelude::{Commands, Component, Entity, On, Pointer, Query, Res, ResMut, Resource, Sphere, Transform, Visibility};
use bevy::reflect::Reflect;

use crate::logic::LogicMessage;

pub struct DraggablePlugin;

impl Plugin for DraggablePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_draggables);
        app.add_systems(Update, add_draggables);

        app
            .add_observer(on_drag)
            .add_observer(on_drag_end);
    }
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct DraggableParams {
    sphere_mesh: Handle<Mesh>,
}

#[derive(Component, Reflect)]
#[require(Transform, Visibility)]
pub struct Draggable {
    axis: Dir3,
    slide: bool,
    range: std::ops::RangeInclusive<f32>,
}

fn init_draggables(
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
) {
    let sphere_mesh = meshes.add(Sphere::new(1.0).mesh());
    commands.insert_resource(DraggableParams { sphere_mesh });
}

fn add_draggables(
    // params: Res<DraggableParams>,
    // clickables: Query<(Entity, &Draggable, &mut Transform), Added<Draggable>>,
    draggables: Query<(Entity, &Name), Without<Draggable>>,
    mut commands: Commands,
) {
    for (e, n) in draggables.iter() {
        if n.as_str() == "i-painting" {
            commands.entity(e).insert(Draggable { axis: Dir3::X, slide: true, range: -10.0..=10.0 });
        }
        if n.as_str().starts_with("i-ring") {
            commands.entity(e).insert(Draggable { axis: Dir3::X, slide: false, range: 0.0..=TAU });
        }
        if n.as_str().starts_with("i-endcap") {
            commands.entity(e).insert(Draggable { axis: Dir3::X, slide: true, range: -2.5..=0.0 });
        }
    }
}

fn on_drag(
    event: On<Pointer<Drag>>,
    pointer_state: Res<PointerState>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut draggables: Query<(&Draggable, &mut Transform, &GlobalTransform)>,
) {
    // Only check entities that have a Draggable component
    let Ok((draggable, mut transform, global_transform)) = draggables.get_mut(event.entity)
        else { return };

    // Need to know the original mesh that was picked, which may be different from the one with the Draggable component
    let original_entity = event.original_event_target();

    let res: Result<(), &str> = (move ||{
        // Get the hit data for where the cursor button was first pressed, and the dragging entry for where it is now
        let pbs = pointer_state.get(event.pointer_id, event.button).ok_or("missing state for pointer")?;
        let (_, _, hit) = pbs.pressing.get(&original_entity).ok_or("missing pressing state for entity")?;
        let de = pbs.dragging.get(&original_entity).ok_or("missing dragging state for entity")?;

        // Recreate the ray for where the cursor has been dragged to, and use the hit depth to find a point along it
        let (camera, cam_gt) = cameras.get(hit.camera).map_err(|_| "camera not found")?;
        let ray = camera.viewport_to_world(&cam_gt, de.latest_pos).map_err(|_| "can't map position to world")?;
        let dragged_to = ray.get_point(hit.depth);

        //TODO use the original transform and the difference between original_pos and dragged_to,
        // so we aren't always moving the origin of the dragged object directly to the cursor
        //let original_pos = hit.position.ok_or("missing position on hit data")?;

        //info!("drag: {:?}, dragged_to: {:?}", event.entity, dragged_to);
        if draggable.slide {
            let mut rel_pos = global_transform.affine().inverse().transform_vector3(dragged_to);
            rel_pos /= transform.scale;
            transform.translation.x = rel_pos.x;//.clamp(*draggable.range.start(), *draggable.range.end());
        } else {
            transform.rotate_x(event.delta.y * 0.01);
        }
        Ok(())
    })();

    if let Err(err) = res {
        info!("on_drag: {}", err);
    }
}

fn on_drag_end(
    event: On<Pointer<DragEnd>>,
    draggables: Query<&Draggable>,
    mut commands: Commands,
) {
    let Ok(draggable) = draggables.get(event.entity)
    else { return };

    commands.write_message(LogicMessage::Dragged(event.entity));
}
