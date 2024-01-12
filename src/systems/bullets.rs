use bevy::window::PrimaryWindow;
use std::{f32, time::Duration};

use crate::prelude::*;

pub fn handle_bullet_timers(
    mut bullet_q: Query<&mut BulletLifeTimer, With<Bullet>>,
    time: Res<Time>,
) {
    for mut bull_timer in bullet_q.iter_mut() {
        bull_timer.tick(time.delta());
    }
}

pub fn despawn_bullet(
    bullet_q: Query<(&BulletLifeTimer, Entity), With<Bullet>>,
    mut despawn_event: EventWriter<DespawnEventRecursive>,
) {
    for (bullet_timer, bullet_ent) in bullet_q.iter() {
        if bullet_timer.finished() {
            despawn_event.send(DespawnEventRecursive(bullet_ent));
        }
    }
}

pub fn spawn_bullet(
    mut cmds: Commands,
    window_q: Query<&Window, With<PrimaryWindow>>,
    player_q: Query<&Transform, With<Player>>,
    mouse_button: Res<Input<MouseButton>>,
    char_texture: Res<AsciiSpriteSheet>,
    mut bullet_timer: ResMut<BulletSpawnTimer>,
    time: Res<Time>,
) {
    bullet_timer.tick(time.delta());

    if bullet_timer.finished() {
        let player_pos = player_q.single().translation.truncate();

        let window = window_q.single();
        let window_size = Vec2::new(
            window.physical_width() as f32,
            window.physical_height() as f32,
        );

        if let Some(mouse_pos) = window.cursor_position() {
            if mouse_button.pressed(MouseButton::Left) {
                // Reamap from window space to a vector around the player.
                let mut mouse_pos = mouse_pos - window_size / 2.;
                mouse_pos.y = -mouse_pos.y;
                let delta = mouse_pos.y.atan2(mouse_pos.x) - f32::consts::FRAC_PI_2;
                mouse_pos = mouse_pos.normalize_or_zero();

                let size = Vec2::new(15., 30.);
                let translation = player_pos + mouse_pos * (PLAYER_SIZE / 2. + 10.);
                let orient = Quat::from_rotation_z(delta);
                let rotation = Quat::IDENTITY * orient;
                let target = mouse_pos;
                let bullet_speed = BULLET_SPEED;

                cmds.spawn(BulletBundle::new(
                    size,
                    char_texture,
                    translation,
                    rotation,
                    target,
                    bullet_speed,
                ));

                // Reset TIMER
                bullet_timer.reset();
            }
        }
    }
}

pub fn handle_enemy_bullet_coll(
    mut collision_events: EventReader<CollisionEvent>,
    bullet_q: Query<(&Transform, Entity), (With<Bullet>, Without<Enemy>)>,
    mut enemy_q: Query<
        (
            &mut Health,
            &mut EnemyObjective,
            &mut ChangeStateTimer,
            &mut UnchangableTimer,
            &mut FollowTimer,
            &mut EnemyShotTimer,
        ),
        With<Enemy>,
    >,
    mut despawn_event: EventWriter<DespawnEventRecursive>,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Stopped(ent_a, ent_b, _) = event {
            // Find bullet and enemy or exit early.
            let mut other_ent = ent_b;
            let Ok(find_bullet) = bullet_q.get(*ent_a).or_else(|_| {
                other_ent = ent_a;
                bullet_q.get(*ent_b)
            }) else {
                return;
            };
            let Ok(find_enemy) = enemy_q.get_mut(*other_ent) else {
                return;
            };
            let (bullet_pos, bullet_ent) = find_bullet;
            let (
                mut enemy_hp,
                mut objective,
                mut change_timer,
                mut unchangable_timer,
                mut follow_timer,
                mut shot_timer,
            ) = find_enemy;

            *shot_timer = EnemyShotTimer::new(ENEMY_STAGGER_TIME);
            enemy_hp.current -= 1;
            if unchangable_timer.is_none() {
                match *objective {
                    EnemyObjective::Bounce => {
                        // Start timer since we will switch the objective below.
                        *change_timer = ChangeStateTimer::new(ENEMY_CHANGE_TIME);
                        objective.switch();
                        // Reset follow timer if started.
                        follow_timer.take();
                        *unchangable_timer = UnchangableTimer::new(Duration::from_millis(50));
                    }
                    EnemyObjective::FollowPlayer => {
                        // vel.linvel = bullet_vel.linvel.normalize_or_zero() * ENEMY_SPEED * 10.;
                    }
                }
            }
            despawn_event.send(DespawnEventRecursive(bullet_ent));
        }
    }
}
