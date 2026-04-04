mod action;
mod check;
mod state;

use bevy::prelude::*;

use crate::logic::check::check_logic;
use crate::logic::state::LogicState;

pub struct LogicPlugin;

impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<LogicMessage>();
        app.init_resource::<LogicState>();
        app.add_systems(Update, check_logic);
    }
}

#[derive(Debug, Message)]
pub enum LogicMessage {
    CreatedPuzzle(Entity),
    Clicked(Entity, bool),
    Dragged(Entity),
}
