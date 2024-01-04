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
    tile_query: Query<(Entity, &GameMapTile)>,
    tilemap_q: Query<&TileStorage>,
) {
    let tile_storage = tilemap_q
        .get_single()
        .expect("There is more than one TILEMAP present in the World!");

    for (mut damping, transform, maybe_player) in damp_query.iter_mut() {
        let (x, y) = (
            (((transform.translation.x + MAP_SIZE_PX.x / 2.0) / TILE_SIZE.x).floor() as u32),
            (((transform.translation.y + MAP_SIZE_PX.y / 2.0) / TILE_SIZE.y).floor() as u32),
        );
        let tile_at_pos = tile_storage
            .get(&TilePos::new(x, y))
            .expect("Entity is at invalid position!");

        let (_, noise_val) = tile_query
            .iter()
            .find(|(ent, _)| ent == &tile_at_pos)
            .unwrap();

        let base_damping = if maybe_player.is_some() {
            PLAYER_DAMPING
        } else {
            ENEMY_DAMPING
        };
        if **noise_val < 0.2 {
            damping.linear_damping = base_damping;
        } else if **noise_val < 0.3 {
            damping.linear_damping = base_damping * 2.;
        } else if **noise_val < 1.0 {
            damping.linear_damping = base_damping * 4.0;
        }
    }
}
