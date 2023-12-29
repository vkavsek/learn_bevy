use rand::{thread_rng, Rng};

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

pub fn enemy_follow_player(
    mut enemy_query: Query<(&mut Velocity, &EnemyObjective, &Transform), With<Enemy>>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    let player_pos = player_query.single();
    for (mut vel, objective, pos) in enemy_query.iter_mut() {
        if matches!(objective, EnemyObjective::FollowPlayer) {
            let new_pos = player_pos.translation - pos.translation;
            **vel = new_pos.truncate().normalize();
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn enemy_random_movement(
    mut enemy_query: Query<
        (&mut Velocity, &EnemyObjective),
        (Changed<EnemyObjective>, With<Enemy>),
    >,
) {
    let mut rng = thread_rng();
    for (mut vel, objective) in enemy_query.iter_mut() {
        if matches!(objective, EnemyObjective::Bounce) {
            let (x, y) = (rng.gen_range(-1f32..1.0), rng.gen_range(-1f32..1.0));
            **vel = Vec2::new(x, y).normalize();
        }
    }
}
