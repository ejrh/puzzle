use bevy::log::info;
use bevy::prelude::{Entity, Resource};

use crate::logic::action::{Constant, Instruction};

#[derive(Debug, Resource)]
pub struct LogicState {
    puzzle_id: Entity,
    current_zone: String,
}

impl Default for LogicState {
    fn default() -> Self {
        LogicState {
            puzzle_id: Entity::PLACEHOLDER,
            current_zone: "".into(),
        }
    }
}

impl LogicState {
    pub fn created(&mut self, entity: Entity) -> Vec<Instruction> {
        self.puzzle_id = entity;
        self.current_zone = "z-main".into();
        vec![
            Instruction::Lookup("z-main".into()),
            Instruction::Constant(Constant::Float(1.0)),
            Instruction::MoveToZone,
        ]
    }

    pub fn clicked(&mut self, entity: Entity, name: &str, primary: bool) -> Vec<Instruction> {
        if primary {
            if self.current_zone == "z-main" && (name == "z-hanging-key" || name == "z-chest") {
                self.current_zone = name.into();
                return vec![
                    Instruction::Constant(Constant::Entity(entity)),
                    Instruction::Constant(Constant::Float(4.0)),
                    Instruction::MoveToZone,
                ];
            }

            if self.current_zone == "z-hanging-key" && name == "i-key" {
                info!("clicked key");
            }
        } else {
            if self.current_zone == "z-hanging-key" || name == "z-chest" {
                self.current_zone = "z-main".into();

                return vec![
                    Instruction::Lookup("z-main".into()),
                    Instruction::Constant(Constant::Float(2.0)),
                    Instruction::MoveToZone,
                ];
            }
        }

        Vec::new()
    }
}
