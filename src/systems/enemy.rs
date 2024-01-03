use crate::prelude::*;
use rand::{thread_rng, Rng};

pub fn setup_enemies(mut cmds: Commands, char_texture: Res<AsciiSpriteSheet>) {
    let mut rng = thread_rng();
    let half_s = MAP_SIZE_PX / 2.;

    for _n in 0..NUM_ENEMIES {
        let rng_x = rng.gen_range((-half_s + ENEMY_SIZE + 1.)..(half_s - ENEMY_SIZE - 1.));
        let rng_y = rng.gen_range((-half_s + ENEMY_SIZE + 1.)..(half_s - ENEMY_SIZE - 1.));
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
        });
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
    mut enemy_query: Query<(&mut Velocity, &Transform, &FollowTimer), With<Enemy>>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    let player_pos = player_query.single();
    for (mut vel, pos, f_timer) in enemy_query.iter_mut() {
        if f_timer.is_some() {
            let new_vel = player_pos.translation - pos.translation;
            vel.linvel = new_vel.truncate().normalize() * ENEMY_SPEED;
        }
    }
}
