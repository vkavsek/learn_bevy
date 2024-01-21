use bevy::window::PrimaryWindow;
use rand::{thread_rng, Rng};
use std::{f32, time::Duration};

use crate::prelude::*;

pub fn handle_bullet_timers(
    mut bullet_q: Query<&mut BulletLifeTimer, With<Bullet>>,
    mut bullet_timer: ResMut<BulletSpawnTimer>,
    time: Res<Time>,
) {
    bullet_timer.tick(time.delta());
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
    player_q: Query<(&Transform, &GunType), With<Player>>,
    mouse_button: Res<Input<MouseButton>>,
    char_texture: Res<AsciiSpriteSheet>,
    mut bullet_timer: ResMut<BulletSpawnTimer>,
) {
    let (player_pos, gun_type) = player_q.single();
    let player_pos = player_pos.translation.truncate();
    if bullet_timer.finished() {
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

                spawn_bullet_type(
                    &mut cmds,
                    gun_type,
                    char_texture.clone(),
                    player_pos,
                    mouse_pos,
                    delta,
                );
                // Reset TIMER
                bullet_timer.reset();
            }
        }
    }
}

fn spawn_bullet_type(
    cmds: &mut Commands,
    gun_type: &GunType,
    char_texture: Handle<TextureAtlas>,
    player_pos: Vec2,
    mouse_pos: Vec2,
    delta: f32,
) {
    let target = mouse_pos;
    let size = Vec2::new(10., 20.);
    let translation = player_pos + mouse_pos * (PLAYER_SIZE / 2. + 10.);
    let rotation = Quat::IDENTITY * Quat::from_rotation_z(delta);

    match gun_type {
        GunType::Pistol => {
            cmds.spawn(BulletBundle::new(
                size,
                char_texture,
                translation,
                rotation,
                target,
                BULLET_SPEED,
            ));
        }
        GunType::Ar => {
            cmds.spawn(BulletBundle::new(
                size,
                char_texture,
                translation,
                rotation,
                target,
                BULLET_SPEED * 1.25,
            ));
        }
        GunType::Shotgun => {
            let size = Vec2::new(10., 10.);
            let num_bullets = 20;
            for _ in 0..num_bullets {
                let mut rng = thread_rng();
                let new_delta = delta
                    + rng.gen_range(-3..3) as f32 * 0.04 * f32::consts::PI
                    + f32::consts::FRAC_PI_2;
                let target = Vec2::new(new_delta.cos(), new_delta.sin());
                let translation = player_pos + target * (PLAYER_SIZE / 2. + 10.);
                let rotation = Quat::IDENTITY * Quat::from_rotation_z(new_delta);

                cmds.spawn(BulletBundle::new(
                    size,
                    char_texture.clone(),
                    translation,
                    rotation,
                    target,
                    BULLET_SPEED,
                ));
            }
        }
    }
}

pub fn handle_bullet_coll(
    mut collision_events: EventReader<CollisionEvent>,
    bullet_q: Query<(&Transform, Entity), (With<Bullet>, Without<Enemy>)>,
    mut enemy_q: Query<
        (
            &mut Health,
            &mut EnemyObjective,
            &mut ChangeStateTimer,
            &mut UnchangableTimer,
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
            // DO FX
            let (bullet_pos, bullet_ent) = find_bullet;

            let Ok(find_enemy) = enemy_q.get_mut(*other_ent) else {
                return;
            };
            let (
                mut enemy_hp,
                mut objective,
                mut change_timer,
                mut unchangable_timer,
                mut shot_timer,
            ) = find_enemy;

            *shot_timer = EnemyShotTimer::new(ENEMY_STAGGER_TIME);
            enemy_hp.current -= 1;
            if unchangable_timer.is_none() {
                match *objective {
                    EnemyObjective::Bounce => {
                        // Start timer since we will switch the objective below.
                        *change_timer = ChangeStateTimer::new(ENEMY_CHANGE_TIME);
                        *objective = EnemyObjective::FollowPlayer;
                        *unchangable_timer = UnchangableTimer::new(Duration::from_millis(50));
                    }
                    EnemyObjective::FollowPlayer => {}
                    EnemyObjective::Attack => {}
                }
            }
            // send despawn event for the bullet if it was found
            despawn_event.send(DespawnEventRecursive(bullet_ent));
        }
    }
}
