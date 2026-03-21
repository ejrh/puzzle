use bevy::math::Curve;
use bevy::prelude::{Commands, Component, EaseFunction, EasingCurve, Entity, Query, Res, Time, Transform};

#[derive(Component)]
pub struct MovingTo {
    target: Transform,
    duration: f32,
    elapsed: f32,
}

impl MovingTo {
    pub fn new(target: Transform, duration: f32) -> MovingTo {
        MovingTo { target, duration, elapsed: 0.0 }
    }
}

pub fn update_movement(
    moving_query: Query<(Entity, &mut Transform, &mut MovingTo)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (id, mut transform, mut moving_to) in moving_query {
        moving_to.elapsed += time.delta_secs();
        let portion = (moving_to.elapsed / moving_to.duration).clamp(0.0, 1.0);
        transform.translation = EasingCurve::new(transform.translation, moving_to.target.translation, EaseFunction::QuadraticInOut).sample_unchecked(portion);
        transform.rotation = EasingCurve::new(transform.rotation, moving_to.target.rotation, EaseFunction::QuadraticInOut).sample_unchecked(portion);
        if portion >= 1.0 {
            commands.entity(id).remove::<MovingTo>();
        }
    }
}
