use bevy::app::{App, Plugin, Update};
use bevy::math::Dir3;
use bevy::prelude::{Component, Query, Reflect, Res, Time, Transform, With};

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_items);
    }
}

#[derive(Component, Reflect)]
pub struct Item;

#[derive(Component)]
pub struct Rotating;

fn animate_items(
    items: Query<&mut Transform, With<Rotating>>,
    time: Res<Time>,
) {
    let rate_per_second = 45_f32.to_radians();
    for mut item in items {
        item.rotate_axis(Dir3::Y, rate_per_second * time.delta().as_secs_f32());
    }
}
