use crate::prelude::*;

pub fn player_collisions(
    mut player_q: Query<(&mut TextureAtlasSprite, &Transform), With<Player>>,
    colliding_q: Query<&Transform, (With<HasCollision>, Without<Player>)>,
) {
    let (mut texture_atlas, player_pos) = player_q.single_mut();
    if colliding_q.iter().any(|collide_pos| {
        let player_pos = Vec2::new(player_pos.translation.x, player_pos.translation.y);
        let other_pos = Vec2::new(collide_pos.translation.x, collide_pos.translation.y);
        player_pos.distance(other_pos) <= PLAYER_SPRITE_WIDTH
    }) {
        texture_atlas.color = Color::WHITE;
    } else {
        texture_atlas.color = PLAYER_COLOR;
    }
}
