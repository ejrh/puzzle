use bevy::prelude::ReflectResource;
use bevy::log::info;
use bevy::prelude::{Entity, Resource};
use bevy::reflect::Reflect;

use crate::logic::action::{Constant, Instruction};

#[derive(Debug, Resource, Reflect)]
#[reflect(Resource)]
pub struct LogicState {
    puzzle_id: Entity,
    current_zone: String,
    got_key: bool,
}

impl Default for LogicState {
    fn default() -> Self {
        LogicState {
            puzzle_id: Entity::PLACEHOLDER,
            current_zone: "".into(),
            got_key: false,
        }
    }
}

impl LogicState {
    pub fn created(&mut self, entity: Entity) -> Vec<Instruction> {
        self.puzzle_id = entity;
        self.current_zone = "z-main".into();
        self.move_to_zone("z-main", 1.0)
    }

    pub fn clicked(&mut self, entity: Entity, name: &str, primary: bool) -> Vec<Instruction> {
        if primary {
            if self.current_zone == "z-main" && (name == "z-hanging-key" || name == "z-painting" || name == "z-chest") {
                return self.move_to_zone(name, 4.0);
            }

            if self.current_zone == "z-hanging-key" && name == "i-key" {
                return self.pick_up_key();
            }
        } else {
            if self.current_zone == "z-hanging-key" || self.current_zone == "z-painting" || self.current_zone == "z-chest" {
                return self.move_to_zone("z-main", 2.0);
            }
        }

        Vec::new()
    }

    fn move_to_zone(&mut self, zone_name: &str, duration: f32) -> Vec<Instruction> {
        self.current_zone = zone_name.into();
        vec![
            Instruction::Lookup("camera".into()),
            Instruction::Lookup(zone_name.into()),
            Instruction::Constant(Constant::Float(duration)),
            Instruction::MoveTo,
        ]
    }

    fn pick_up_key(&mut self) -> Vec<Instruction> {
        info!("Picked up key");
        self.got_key = true;
        vec![
            Instruction::Lookup("i-key".into()),
            Instruction::Lookup("camera".into()),
            Instruction::ReparentInPlace,

            Instruction::Lookup("i-key".into()),
            Instruction::RemoveDecoration,

            Instruction::Lookup("i-key".into()),
            Instruction::Lookup("hand".into()),
            Instruction::Constant(Constant::Float(0.5)),
            Instruction::MoveTo,
        ]
    }
}
