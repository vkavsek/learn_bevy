// use rand::{thread_rng, Rng};
//
use crate::prelude::*;

pub fn cam_movement(
    mut camera_q: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    player_q: Query<&Transform, With<Player>>,
) {
    if let Ok(player_pos) = player_q.get_single() {
        for mut camera_pos in camera_q.iter_mut() {
            camera_pos.translation = player_pos.translation;
        }
    }
}

pub fn dynamic_damping(
    mut damp_query: Query<(&mut Damping, &Transform, Option<&Player>)>,
    noise_map: Res<NoiseMapValues>,
) {
    for (mut damping, transform, maybe_player) in damp_query.iter_mut() {
        let (x, y) = (
            (((transform.translation.x + MAP_SIZE_PX / 2.0) / TILE_SIZE).floor() as usize),
            (((transform.translation.y + MAP_SIZE_PX / 2.0) / TILE_SIZE).floor() as usize),
        );
        let val = noise_map.get_value(x, y).abs();
        let base_damping = if maybe_player.is_some() {
            PLAYER_DAMPING
        } else {
            ENEMY_DAMPING
        };
        if val < 0.2 {
            damping.linear_damping = base_damping;
        } else if val < 0.3 {
            damping.linear_damping = base_damping * 2.;
        } else if val < 1.0 {
            damping.linear_damping = base_damping * 4.0;
        }
    }
}

// pub fn enemy_follow_player(
//     mut enemy_query: Query<(&mut Vel, &EnemyObjective, &Transform), With<Enemy>>,
//     player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
// ) {
//     let player_pos = player_query.single();
//     for (mut vel, objective, pos) in enemy_query.iter_mut() {
//         if matches!(objective, EnemyObjective::FollowPlayer) {
//             let new_pos = player_pos.translation - pos.translation;
//             **vel = new_pos.truncate().normalize();
//         }
//     }
// }
//
// #[allow(clippy::type_complexity)]
// pub fn enemy_random_movement(
//     mut enemy_query: Query<(&mut Vel, &EnemyObjective), (Changed<EnemyObjective>, With<Enemy>)>,
// ) {
//     let mut rng = thread_rng();
//     for (mut vel, objective) in enemy_query.iter_mut() {
//         if matches!(objective, EnemyObjective::Bounce) {
//             let (x, y) = (rng.gen_range(-1f32..1.0), rng.gen_range(-1f32..1.0));
//             **vel = Vec2::new(x, y).normalize();
//         }
//     }
//
// }
