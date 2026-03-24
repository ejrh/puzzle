use bevy::log::{info, warn};
use bevy::prelude::{BevyError, ChildOf, Commands, Entity, GlobalTransform, In, MessageReader, Name, Query, ResMut, Transform, World};
use crate::item::Rotating;
use crate::logic::action::{Constant, Instruction};
use crate::logic::LogicMessage;
use crate::logic::state::LogicState;
use crate::utils::movement::MovingTo;

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
            Instruction::MoveTo => {
                let Some(Constant::Float(duration)) = stack.pop()
                    else { warn!("Stack does not contain a float"); return; };
                let Some(Constant::Entity(target_id)) = stack.pop()
                    else { warn!("Stack does not contain an entity"); return; };
                let Some(Constant::Entity(entity_id)) = stack.pop()
                    else { warn!("Stack does not contain an entity"); return; };
                world.commands().run_system_cached_with(move_to, (entity_id, target_id, duration));
            },
            Instruction::ReparentInPlace => {
                let Some(Constant::Entity(new_parent_id)) = stack.pop()
                else { warn!("Stack does not contain an entity"); return; };
                let Some(Constant::Entity(entity_id)) = stack.pop()
                else { warn!("Stack does not contain an entity"); return; };
                world.commands().run_system_cached_with(reparent_in_place, (entity_id, new_parent_id));
            },
            Instruction::RemoveDecoration => {
                let Some(Constant::Entity(entity_id)) = stack.pop()
                else { warn!("Stack does not contain an entity"); return; };
                world.commands().run_system_cached_with(remove_decoration, entity_id);
            },
        }
    }
}

pub fn move_to(
    input: In<(Entity, Entity, f32)>,
    transforms: Query<&Transform>,
    mut commands: Commands,
) -> Result<(), BevyError> {
    let (entity_id, target_id, duration) = *input;
    let target = transforms.get(target_id)?;

    commands.entity(entity_id).insert(
        MovingTo::new(*target, duration),
    );

    Ok(())
}

pub fn reparent_in_place(
    input: In<(Entity, Entity)>,
    transforms: Query<&GlobalTransform>,
    mut commands: Commands,
) -> Result<(), BevyError> {
    let (entity_id, new_parent_id) = *input;

    let current_transform = transforms.get(entity_id)?;
    let new_parent_transform = transforms.get(new_parent_id)?;

    let local_transform = current_transform.reparented_to(new_parent_transform);

    commands.entity(entity_id).insert((
        local_transform,
        ChildOf(new_parent_id),
    ));

    Ok(())
}

pub fn remove_decoration(
    input: In<Entity>,
    mut commands: Commands,
) -> Result<(), BevyError> {
    let entity_id = *input;

    commands.entity(entity_id).remove::<(
        Rotating,
    )>();

    Ok(())
}
