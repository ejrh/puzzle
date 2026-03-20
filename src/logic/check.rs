use bevy::camera::Camera;
use bevy::log::{info, warn};
use bevy::prelude::{BevyError, Commands, Entity, In, MessageReader, Name, Query, ResMut, Single, With, World};

use crate::logic::action::{Constant, Instruction};
use crate::logic::LogicMessage;
use crate::logic::state::LogicState;
use crate::utils::camera_move::CameraMovingTo;
use crate::zone::ZoneCamera;

pub fn check_logic(
    mut messages: MessageReader<LogicMessage>,
    names: Query<&Name>,
    mut state: ResMut<LogicState>,
    mut commands: Commands,
) {
    let mut actions = Vec::new();

    for msg in messages.read() {
        info!("Checking logic message: {:?}", msg);
        let new_actions = match *msg {
            LogicMessage::CreatedPuzzle(entity) => state.created(entity),
            LogicMessage::Clicked(entity, primary) => {
                let name = names.get(entity).map(|n| n.as_str()).unwrap_or("?");
                info!("Clicked entity: {:?} {}", entity, name);

                state.clicked(entity, name, primary)
            }
        };
        actions.extend(new_actions);
    }

    commands.run_system_cached_with(run_actions, actions);
}

pub fn run_actions(
    actions: In<Vec<Instruction>>,
    world: &mut World,
) {
    let mut stack = Vec::new();

    for action in actions.iter() {
        info!("Performing action: {:?}", action);
        match action {
            Instruction::Lookup(name) => {
                let Some(id) = world.query::<(Entity, &Name)>().iter(world)
                    .find_map(|(id, n)| (n.as_str() == name).then_some(id)).iter().cloned().next()
                else { warn!("Can't find entity for name {}", name); return; };
                stack.push(Constant::Entity(id));
            },
            Instruction::Constant(constant) => {
                stack.push(constant.clone());
            },
            Instruction::MoveToZone => {
                let Some(Constant::Float(duration)) = stack.pop()
                    else { warn!("Stack does not contain a float"); return; };
                let Some(Constant::Entity(zone_id)) = stack.pop()
                    else { warn!("Stack does not contain an entity"); return; };
                world.commands().run_system_cached_with(move_to_zone, (zone_id, duration));
            },
        }
    }
}

pub fn move_to_zone(
    input: In<(Entity, f32)>,
    zones: Query<&ZoneCamera>,
    camera: Single<Entity, With<Camera>>,
    mut commands: Commands,
) -> Result<(), BevyError> {
    let (zone_id, duration) = *input;
    let zone_camera = zones.get(zone_id)?;
    let camera_id = *camera;

    commands.entity(camera_id).insert(
        CameraMovingTo::new(zone_camera.0, duration),
    );

    Ok(())
}
