use bevy::math::Curve;
use bevy::prelude::{Commands, Component, EaseFunction, EasingCurve, Entity, Query, Res, Time, Transform};

#[derive(Component)]
pub struct CameraMovingTo {
    target: Transform,
    duration: f32,
    elapsed: f32,
}

impl CameraMovingTo {
    pub fn new(target: Transform, duration: f32) -> CameraMovingTo {
        CameraMovingTo { target, duration, elapsed: 0.0 }
    }
}

pub fn camera_move(
    cameras: Query<(Entity, &mut Transform, &mut CameraMovingTo)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (id, mut transform, mut moving_to) in cameras {
        moving_to.elapsed += time.delta_secs();
        let portion = (moving_to.elapsed / moving_to.duration).clamp(0.0, 1.0);
        transform.translation = EasingCurve::new(transform.translation, moving_to.target.translation, EaseFunction::QuadraticInOut).sample_unchecked(portion);
        transform.rotation = EasingCurve::new(transform.rotation, moving_to.target.rotation, EaseFunction::QuadraticInOut).sample_unchecked(portion);
        if portion >= 1.0 {
            commands.entity(id).remove::<CameraMovingTo>();
        }
    }
}
