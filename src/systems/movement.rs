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

pub fn player_movement(
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &Movement), With<Player>>,
) {
    let (mut pos, movement) = player_query.single_mut();

    // let half_width = PLAYER_SPRITE_WIDTH / 2.;
    // let half_height = PLAYER_SPRITE_HEIGHT / 2.;

    pos.translation.x += movement.vel.x * PLAYER_SPEED * time.delta_seconds();
    // .clamp(-HALF_WIDTH + half_width, HALF_WIDTH - half_width);
    pos.translation.y += movement.vel.y * PLAYER_SPEED * time.delta_seconds();
    // .clamp(-HALF_HEIGHT + half_height, HALF_HEIGHT - half_height);
}

pub fn enemy_movement(
    time: Res<Time>,
    mut enemy_query: Query<(&mut Transform, &mut Movement), With<Enemy>>,
) {
    for (mut trans, mut movement) in &mut enemy_query {
        let half_sprite_width = ENEMY_SPRITE_WIDTH / 2.;
        let x = trans.translation.x;
        let y = trans.translation.y;
        if x <= -HALF_WIDTH + half_sprite_width || x >= HALF_WIDTH - half_sprite_width {
            movement.vel.x *= -1.;
        }
        if y <= -HALF_HEIGHT + half_sprite_width || y >= HALF_HEIGHT - half_sprite_width {
            movement.vel.y *= -1.;
        }

        trans.translation.x += movement.vel.x * ENEMY_SPEED * time.delta_seconds();
        trans.translation.y += movement.vel.y * ENEMY_SPEED * time.delta_seconds();
    }
}
