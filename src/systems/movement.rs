#![allow(clippy::type_complexity)]
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

pub fn handle_player_enemy_collisions(
    mut collision_events: EventReader<CollisionEvent>,
    mut player_q: Query<(&mut Health, Entity), (With<Player>, Without<Enemy>)>,
    mut enemy_q: Query<
        (
            &mut Health,
            &mut EnemyObjective,
            &mut ChangeStateTimer,
            &mut FollowTimer,
            Entity,
        ),
        With<Enemy>,
    >,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Stopped(e_ent, p_ent, _) = event {
            let player = player_q.iter_mut().find(|(_, ent)| ent == p_ent);
            let enemy = enemy_q.iter_mut().find(|(_, _, _, _, ent)| ent == e_ent);
            if let Some((mut enemy_hp, mut objective, mut chst_timer, mut follow_timer, _)) = enemy
            {
                if let Some((mut player_hp, _)) = player {
                    if matches!(*objective, EnemyObjective::FollowPlayer) {
                        player_hp.current -= 1;
                    }
                    **follow_timer = None;
                    enemy_hp.current -= 1;

                    // info!(
                    //     "Player hit: HP: {:?}\nEnemy hit: HP: {:?}, OBJ: {:?}",
                    //     player_hp, enemy_hp, objective
                    // );
                    chst_timer.change_state(ENEMY_CHANGE_DELAY);
                    objective.switch();
                }
            }
        }
    }
}

pub fn tick_enemy_timers(
    mut enemy_q: Query<(&mut ChangeStateTimer, &mut FollowTimer), With<Enemy>>,
    time: Res<Time>,
) {
    for (mut change_timer, mut follow_timer) in enemy_q.iter_mut() {
        (*change_timer)
            .as_mut()
            .map(|timer| timer.tick(time.delta()));
        (*follow_timer)
            .as_mut()
            .map(|timer| timer.tick(time.delta()));

        if let Some(timer) = (**change_timer).as_ref() {
            if timer.just_finished() {
                info!("CHANGE DELAY TIMER JUST FINISHED");
            }
        }
        if let Some(timer) = (**follow_timer).as_ref() {
            if timer.just_finished() {
                info!("FOLLOW TIMER JUST FINISHED");
            }
        }
    }
}

pub fn handle_enemy_objective_timers(
    mut enemy_query: Query<
        (&mut EnemyObjective, &mut ChangeStateTimer, &mut FollowTimer),
        With<Enemy>,
    >,
) {
    for (mut enemy_obj, mut change_timer, mut follow_timer) in enemy_query.iter_mut() {
        match *enemy_obj {
            EnemyObjective::FollowPlayer => {
                if let Some(f_timer) = (**follow_timer).as_ref() {
                    if f_timer.just_finished() {
                        *enemy_obj = EnemyObjective::Bounce;
                        (*follow_timer).take();
                    }
                }
                if let Some(ch_timer) = (**change_timer).as_ref() {
                    if ch_timer.just_finished() {
                        *follow_timer = FollowTimer::new(ENEMY_FOLLOW_TIME);
                        (*change_timer).take();
                    }
                }
            }
            EnemyObjective::Bounce => {}
        }
    }
}

// pub fn enemy_follow_player(
//     mut enemy_query: Query<
//         (&mut Velocity, &EnemyObjective, &Transform, &FollowTimer),
//         (With<Enemy>, Changed<EnemyObjective>),
//     >,
//     player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
// ) {
//     let player_pos = player_query.single();
//     // for (mut vel, objective, pos) in enemy_query.iter_mut() {
//     //     if matches!(objective, EnemyObjective::FollowPlayer(_)) {
//     //         let new_pos = player_pos.translation - pos.translation;
//     //     }
//     // }
// }
