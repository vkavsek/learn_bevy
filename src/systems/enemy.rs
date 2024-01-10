use crate::prelude::*;
use rand::{thread_rng, Rng};
use std::time::Duration;

pub fn setup_enemies(mut cmds: Commands, char_texture: Res<AsciiSpriteSheet>) {
    let mut rng = thread_rng();
    let half_s = MAP_SIZE_PX / 2.;

    for _n in 0..NUM_ENEMIES {
        let rng_x = rng.gen_range((-half_s.x + ENEMY_SIZE + 1.)..(half_s.x - ENEMY_SIZE - 1.));
        let rng_y = rng.gen_range((-half_s.y + ENEMY_SIZE + 1.)..(half_s.y - ENEMY_SIZE - 1.));
        let healthbar = cmds
            .spawn(HealthBarBundle::new(Color::RED, Vec2::new(50., 7.5)))
            .id();

        cmds.spawn(EnemyBundle {
            enemy: Enemy,
            enemy_type: Default::default(),
            health: Health::init(ENEMY_HEALTH, ENEMY_HEALTH),
            spritesheet_bundle: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    color: Color::BLACK,
                    index: 42,
                    custom_size: Some(Vec2::from((ENEMY_SIZE, ENEMY_SIZE))),
                    ..default()
                },
                texture_atlas: char_texture.clone(),
                transform: Transform::from_xyz(rng_x, rng_y, 0.2),
                ..default()
            },
            // x: rng.gen_range(-1.0..1.0),
            // y: rng.gen_range(-1.0..1.0),
            ..default()
        })
        .add_child(healthbar);
    }
}

pub fn handle_enemy_timers(
    mut enemy_q: Query<
        (
            &mut EnemyObjective,
            &mut ChangeStateTimer,
            &mut UnchangableTimer,
            &mut FollowTimer,
        ),
        With<Enemy>,
    >,
    time: Res<Time>,
) {
    for (mut enemy_obj, mut change_timer, mut unchangeble_timer, mut follow_timer) in
        enemy_q.iter_mut()
    {
        (*change_timer)
            .as_mut()
            .map(|timer| timer.tick(time.delta()));
        (*follow_timer)
            .as_mut()
            .map(|timer| timer.tick(time.delta()));

        (*unchangeble_timer)
            .as_mut()
            .map(|timer| timer.tick(time.delta()));

        if let Some(unch_timer) = (**unchangeble_timer).as_ref() {
            if unch_timer.finished() {
                unchangeble_timer.take();
            }
        }

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

pub fn handle_enemy_player_coll(
    mut collision_events: EventReader<CollisionEvent>,
    mut player_q: Query<(&mut Health, Entity), (With<Player>, Without<Enemy>)>,
    mut enemy_q: Query<
        (
            &mut Health,
            &mut EnemyObjective,
            &mut ChangeStateTimer,
            &mut UnchangableTimer,
            &mut FollowTimer,
            Entity,
        ),
        With<Enemy>,
    >,
) {
    let (mut player_hp, player_ent) = player_q.single_mut();
    for event in collision_events.read() {
        if let CollisionEvent::Stopped(ent_a, ent_b, _) = event {
            let (_found_player_ent, other_entity) = if ent_a == &player_ent {
                (ent_a, ent_b)
            } else if ent_b == &player_ent {
                (ent_b, ent_a)
            } else {
                return;
            };

            let find_enemy = enemy_q.get_mut(*other_entity);

            if let Ok((
                mut enemy_hp,
                mut objective,
                mut change_timer,
                mut unchangable_timer,
                mut follow_timer,
                _,
            )) = find_enemy
            {
                if unchangable_timer.is_none() {
                    match *objective {
                        EnemyObjective::FollowPlayer => {
                            player_hp.current -= 1;
                            // Take from timer if enemy will get deactivated
                            change_timer.take();
                        }
                        EnemyObjective::Bounce => {
                            // Start timer since we will switch the objective below.
                            *change_timer = ChangeStateTimer::new(ENEMY_CHANGE_TIME);
                        }
                    }
                    enemy_hp.current -= 1;
                    objective.switch();
                    // Reset follow timer if started.
                    follow_timer.take();
                    *unchangable_timer = UnchangableTimer::new(Duration::from_millis(50));
                }
            }
        }
    }
}

pub fn change_enemy_color(
    mut enemy_query: Query<(&EnemyObjective, &mut TextureAtlasSprite), With<Enemy>>,
) {
    for (objective, mut texture_atlas) in enemy_query.iter_mut() {
        match objective {
            EnemyObjective::FollowPlayer => texture_atlas.color = Color::RED,
            EnemyObjective::Bounce => texture_atlas.color = Color::BLACK,
        }
    }
}

pub fn enemy_follow_player(
    mut enemy_query: Query<(&mut Velocity, &Transform, &FollowTimer, &Damping), With<Enemy>>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    let player_pos = player_query.single();
    for (mut vel, pos, f_timer, damping) in enemy_query.iter_mut() {
        if f_timer.is_some() {
            let new_vel = player_pos.translation - pos.translation;
            vel.linvel = new_vel.truncate().normalize() * (ENEMY_SPEED - damping.linear_damping);
        }
    }
}

pub fn despawn_enemy(
    mut cmds: Commands,
    enemy_q: Query<(Entity, &Health), (With<Enemy>, Changed<Health>)>,
) {
    for (ent, health) in enemy_q.iter() {
        if health.current <= 0 {
            cmds.entity(ent).despawn_recursive();
        }
    }
}
