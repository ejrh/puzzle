use bevy::log::{info, warn};
use bevy::prelude::{BevyError, ChildOf, Commands, Entity, GlobalTransform, In, MessageReader, Name, Query, Res, ResMut, Transform, World};
use bevy::time::Time;

use crate::item::Rotating;
use crate::logic::action::{Constant, Instruction, Stack};
use crate::logic::LogicMessage;
use crate::logic::state::LogicState;
use crate::utils::movement::MovingTo;

const LOGIC_REST_SECS: f32 = 0.5;

pub fn check_logic(
    mut messages: MessageReader<LogicMessage>,
    names: Query<&Name>,
    mut state: ResMut<LogicState>,
    time: Res<Time>,
    mut commands: Commands,
) {
    if messages.is_empty() { return; }

    let mut resting = time.elapsed_secs() - state.last_action < LOGIC_REST_SECS;

    let mut actions = Vec::new();

    for msg in messages.read() {
        info!("Checking logic message: {:?}", msg);
        let new_actions = match *msg {
            LogicMessage::CreatedPuzzle(entity) => state.created(entity),
            LogicMessage::Clicked(entity, primary) => {
                if resting {
                    info!("Ignoring click while resting");
                    continue;
                }
                let name = names.get(entity).map(|n| n.as_str()).unwrap_or("?");
                info!("Clicked entity: {:?} {}", entity, name);

                state.clicked(entity, name, primary)
            }
        };
        if !new_actions.is_empty() {
            info!("Generated {} new actions", new_actions.len());
            state.last_action = time.elapsed_secs();
            resting = true;
        }
        actions.extend(new_actions);
    }

    commands.run_system_cached_with(run_actions, actions);
}

pub fn run_actions(
    actions: In<Vec<Instruction>>,
    world: &mut World,
) -> Result<(), BevyError> {
    let mut stack = Stack::default();

    for action in actions.iter() {
        info!("Performing action: {:?}", action);
        match action {
            Instruction::Lookup(name) => {
                let Some(id) = world.query::<(Entity, &Name)>().iter(world)
                    .find_map(|(id, n)| (n.as_str() == name).then_some(id)).iter().cloned().next()
                else { warn!("Can't find entity for name {}", name); return Err("bad".into()); };
                stack.push(Constant::Entity(id));
            },
            Instruction::Constant(constant) => {
                stack.push(constant.clone());
            },
            Instruction::MoveTo => {
                let (entity_id, target_id, duration) = stack.pop3()?;
                world.commands().run_system_cached_with(move_to, (entity_id, target_id, duration));
            },
            Instruction::ReparentInPlace => {
                let (entity_id, new_parent_id) = stack.pop2()?;
                world.commands().run_system_cached_with(reparent_in_place, (entity_id, new_parent_id));
            },
            Instruction::RemoveDecoration => {
                let entity_id = stack.pop1()?;
                world.commands().run_system_cached_with(remove_decoration, entity_id);
            },
        }
    }
    
    Ok(())
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
