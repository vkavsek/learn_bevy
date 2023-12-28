use bevy::sprite::collide_aabb::{collide, Collision};

use crate::prelude::*;

#[allow(clippy::type_complexity)]
pub fn player_enemy_collision(
    mut query: Query<(&mut Velocity, &Transform, &Size), With<Player>>,
    mut collider_query: Query<
        (&mut Velocity, &Transform, &Size),
        (With<Collider>, Without<Player>),
    >,
    // mut collision_events: EventWriter<CollisionEvent>,
) {
    let (mut _vel, pos, size) = query.single_mut();

    for (mut collider_vel, collider_pos, coll_size) in collider_query.iter_mut() {
        let collision = collide(
            pos.translation,
            **size,
            collider_pos.translation,
            **coll_size,
        );
        if let Some(collision) = collision {
            match collision {
                Collision::Left | Collision::Right => collider_vel.x *= -1.,
                Collision::Top | Collision::Bottom => collider_vel.y *= -1.,
                Collision::Inside => {
                    collider_vel.x *= -1.;
                    collider_vel.y *= -1.;
                }
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn enemy_static_collision(
    mut query: Query<(&mut Velocity, &Transform, &Size), With<Enemy>>,
    static_collider_query: Query<(&Transform, &Size), (With<Collider>, Without<Velocity>)>,
) {
    for (mut vel, pos, size) in query.iter_mut() {
        for (static_pos, static_size) in static_collider_query.iter() {
            let collision = collide(
                pos.translation,
                **size,
                static_pos.translation,
                **static_size,
            );

            if let Some(collision) = collision {
                match collision {
                    Collision::Left | Collision::Right => vel.x *= -1.,
                    Collision::Top | Collision::Bottom => vel.y *= -1.,
                    Collision::Inside => {
                        vel.x *= -1.;
                        vel.y *= -1.;
                    }
                }
            }
        }
    }
}
