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

///  NOTE: If this is suspected to be slow you could access the noise value directly from NoiseMap
pub fn dynamic_damping(
    mut player_query: Query<
        (&mut Damping, &Transform, &mut PlayerNoiseDebug),
        (With<Player>, Without<Enemy>),
    >,
    mut enemy_query: Query<(&mut Damping, &Transform), (With<Enemy>, Without<Player>)>,
    tile_query: Query<(Entity, &GameMapTile)>,
    tilemap_q: Query<&TileStorage>,
) {
    let tile_storage = tilemap_q
        .get_single()
        .expect("There is more than one TILEMAP present in the World!");

    for (mut damping, transform, mut player_noise_debug) in player_query.iter_mut() {
        let (x, y) = (
            (((transform.translation.x + MAP_SIZE_PX.x / 2.0) / GRID_SIZE.x).floor() as u32),
            (((transform.translation.y + MAP_SIZE_PX.y / 2.0) / GRID_SIZE.y).floor() as u32),
        );
        let tile_at_pos = tile_storage
            .get(&TilePos::new(x, y))
            .expect("Entity is at invalid position!");
        let noise_val = tile_query
            .get_component::<GameMapTile>(tile_at_pos)
            .unwrap();

        **player_noise_debug = Some(**noise_val);

        let base_damping = PLAYER_DAMPING;
        if noise_val.abs() < 0.2 {
            damping.linear_damping = base_damping;
        } else if noise_val.abs() < 0.3 {
            damping.linear_damping = base_damping * 2.;
        } else if noise_val.abs() < 1.0 {
            damping.linear_damping = base_damping * 4.0;
        }
    }
    for (mut damping, transform) in enemy_query.iter_mut() {
        let (x, y) = (
            (((transform.translation.x + MAP_SIZE_PX.x / 2.0) / GRID_SIZE.x).floor() as u32),
            (((transform.translation.y + MAP_SIZE_PX.y / 2.0) / GRID_SIZE.y).floor() as u32),
        );
        let tile_at_pos = tile_storage
            .get(&TilePos::new(x, y))
            .expect("Entity is at invalid position!");
        let noise_val = tile_query
            .get_component::<GameMapTile>(tile_at_pos)
            .expect("Each tile should have a GameMapTile component!");

        let base_damping = ENEMY_DAMPING;
        if noise_val.abs() < 0.3 {
            damping.linear_damping = base_damping;
        } else if noise_val.abs() < 1.0 {
            damping.linear_damping = base_damping * 2.;
        }
    }
}
