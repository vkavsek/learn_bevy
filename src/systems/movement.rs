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
