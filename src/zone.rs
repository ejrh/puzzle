use bevy::prelude::{Component, Reflect, Transform};

#[derive(Component, Reflect)]
pub struct Zone;

#[derive(Component, Reflect)]
pub struct ZoneCamera(pub Transform);
