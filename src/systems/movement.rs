use crate::prelude::*;

pub fn cam_movement(
    mut camera_q: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    player_q: Query<&Transform, With<Player>>,
) {
    if let Ok(player_pos) = player_q.get_single() {
        if let Ok(mut camera_pos) = camera_q.get_single_mut() {
            camera_pos.translation = player_pos.translation;
        }
    }
}

pub fn apply_velocity(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity, &Speed)>) {
    for (mut pos, movement, speed) in query.iter_mut() {
        let speed = **speed;

        pos.translation.x += movement.x * speed * time.delta_seconds();
        pos.translation.y += movement.y * speed * time.delta_seconds();
    }
}
