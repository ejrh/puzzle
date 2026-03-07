use bevy::prelude::{Commands, Component, Entity, Query, Res, Time, Transform};

#[derive(Component)]
pub struct CameraMovingTo(pub Transform, pub f32);

pub fn camera_move(
    cameras: Query<(Entity, &mut Transform, &mut CameraMovingTo)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (id, mut transform, mut moving_to) in cameras {
        let portion = (time.delta_secs() / moving_to.1).min(1.0);
        transform.translation = transform.translation.lerp(moving_to.0.translation, portion);
        transform.rotation = transform.rotation.slerp(moving_to.0.rotation, portion);
        moving_to.1 -= time.delta_secs();
        if moving_to.1 <= 0.0 {
            commands.entity(id).remove::<CameraMovingTo>();
        }
    }
}
