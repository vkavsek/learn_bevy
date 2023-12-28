use bevy::sprite::collide_aabb::collide;

use crate::prelude::*;

pub fn check_for_collisions_player(
    mut query: Query<(&mut Velocity, &Transform, &Size), With<Player>>,
    collider_query: Query<(&Transform, &Size), With<Collider>>,
    // mut collision_events: EventWriter<CollisionEvent>,
) {
    let (mut _vel, pos, size) = query.single_mut();

    for (collider_pos, coll_size) in collider_query.iter() {
        let collision = collide(
            pos.translation,
            **size,
            collider_pos.translation,
            **coll_size,
        );
        if collision.is_some() {
            info!("Collision");
        }
    }
}
