use std::f32::consts::TAU;
use std::str::FromStr;

use bevy::prelude::{ReflectResource, Transform};
use bevy::log::info;
use bevy::math::EulerRot;
use bevy::prelude::{Entity, Resource};
use bevy::reflect::Reflect;

use crate::logic::action::Instruction;

#[derive(Debug, Resource, Reflect)]
#[reflect(Resource)]
pub struct LogicState {
    pub last_action: f32,
    puzzle_id: Entity,
    current_zone: String,
    got_key: bool,
    codex_rings: [usize; 4],
    codex_unlocked: bool,
}

impl Default for LogicState {
    fn default() -> Self {
        LogicState {
            last_action: 0.0,
            puzzle_id: Entity::PLACEHOLDER,
            current_zone: "".into(),
            got_key: false,
            codex_rings: [0; 4],
            codex_unlocked: false,
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
            if self.current_zone == "z-main" && (name == "z-hanging-key" || name == "z-painting" || name == "z-chest" || name == "z-cryptex") {
                return self.move_to_zone(name, 4.0);
            }

            if self.current_zone == "z-hanging-key" && name == "i-key" && !self.got_key {
                return self.pick_up_key();
            }
        } else {
            if self.current_zone == "z-hanging-key" || self.current_zone == "z-painting" || self.current_zone == "z-chest" || self.current_zone == "z-cryptex" {
                return self.move_to_zone("z-main", 2.0);
            }
        }

        Vec::new()
    }

    pub fn dragged(&mut self, entity: Entity, name: &str, dragged_to: Transform) -> Vec<Instruction> {
        if name.starts_with("i-ring") {
            return self.move_ring(name, dragged_to)
        }

        Vec::new()
    }

    fn move_to_zone(&mut self, zone_name: &str, duration: f32) -> Vec<Instruction> {
        self.current_zone = zone_name.into();
        vec![
            Instruction::Lookup("camera".into()),
            Instruction::Lookup(zone_name.into()),
            Instruction::GetTransform,
            Instruction::Push(duration.into()),
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
            Instruction::GetTransform,
            Instruction::Push(0.5.into()),
            Instruction::MoveTo,
        ]
    }

    fn move_ring(&mut self, name: &str, dragged_to: Transform) -> Vec<Instruction> {
        let Ok(ring_no) = usize::from_str(&name[6..])
        else { return Vec::new() };

        let angle = dragged_to.rotation.to_euler(EulerRot::XYZ).0;
        let fpos = (angle / TAU + 1.0) * 7.0;
        let pos = (fpos + 0.5) as isize;
        let new_angle = (pos as f32 / 7.0) * TAU;
        let mut new_transform = dragged_to.clone();
        new_transform.rotation = Default::default();
        new_transform.rotate_x(new_angle);

        let corrected_pos = (pos % 7) as usize;
        self.codex_rings[ring_no] = corrected_pos;

        let mut actions = Vec::new();

        if !self.codex_unlocked && self.codex_rings == [1, 2, 3, 4] {
            self.codex_unlocked = true;
            let endcap_transform = Transform::from_xyz(-4.0, 0.0, 0.0);
            actions.extend([
                Instruction::Lookup("i-endcap".into()),
                Instruction::Push(endcap_transform.into()),
                Instruction::Push(0.5.into()),
                Instruction::MoveTo,
            ]);
        }

        actions.extend([
            Instruction::Lookup(name.into()),
            Instruction::Push(new_transform.into()),
            Instruction::Push(0.1.into()),
            Instruction::MoveTo,
        ]);

        actions
    }
}
