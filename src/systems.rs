use rand::{thread_rng, Rng};

use crate::prelude::*;

// NOTE:
//        —————>  SYSTEMS
pub fn setup(mut cmds: Commands, asset_server: Res<AssetServer>) {
    info!("STARTUP");
    cmds.spawn(Camera2dBundle::default());

    // PLAYER SPAWN
    cmds.spawn(PlayerBundle {
        player: Player::init("TODO"),
        sprite_bundle: SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::from((PLAYER_SPRITE_WIDTH, PLAYER_SPRITE_HEIGHT))),
                ..Default::default()
            },
            texture: asset_server.load("player.png"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
        ..Default::default()
    });

    // ENEMY SPAWN
    let hp = 50;
    let mut rng = thread_rng();

    for _n in 0..10 {
        let rng_x = rng.gen_range(
            (-HALF_WIDTH + ENEMY_SPRITE_WIDTH + 1.)..(HALF_WIDTH - ENEMY_SPRITE_WIDTH - 1.),
        );
        let rng_y = rng.gen_range(
            (-HALF_HEIGHT + ENEMY_SPRITE_WIDTH + 1.)..(HALF_HEIGHT - ENEMY_SPRITE_WIDTH - 1.),
        );
        cmds.spawn(EnemyBundle {
            enemy: Enemy,
            enemy_type: Default::default(),
            health: Health::init(hp, hp),
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::from((ENEMY_SPRITE_WIDTH, ENEMY_SPRITE_WIDTH))),
                    ..Default::default()
                },
                texture: asset_server.load("enemy.png"),
                transform: Transform::from_xyz(rng_x, rng_y, 0.0),
                // .with_scale(Vec2::splat(0.1).extend(1.)),
                ..Default::default()
            },
            movement: Movement {
                vel: Vec2 {
                    x: rng.gen_range(-1.0..1.0),
                    y: rng.gen_range(-1.0..1.0),
                }
                .normalize(),
            },
        });
    }
}

/// TODO: Improve input handling
pub fn handle_input(
    keycode_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Movement, With<Player>>,
) {
    let mut movement = player_query.single_mut();

    movement.vel.x = 0.;
    movement.vel.y = 0.;
    // RIGHT
    if keycode_input.any_pressed([KeyCode::D, KeyCode::Right]) {
        movement.vel.x += 1.;
    }
    // LEFT
    if keycode_input.any_pressed([KeyCode::A, KeyCode::Left]) {
        movement.vel.x -= 1.;
    }
    // UP
    if keycode_input.any_pressed([KeyCode::W, KeyCode::Up]) {
        movement.vel.y += 1.;
    }
    // DOWN
    if keycode_input.any_pressed([KeyCode::S, KeyCode::Down]) {
        movement.vel.y -= 1.;
    }
}

pub fn player_movement(
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &Movement), With<Player>>,
) {
    let (mut pos, movement) = player_query.single_mut();

    let half_width = PLAYER_SPRITE_WIDTH / 2.;
    let half_height = PLAYER_SPRITE_HEIGHT / 2.;

    pos.translation.x = (pos.translation.x + movement.vel.x * PLAYER_SPEED * time.delta_seconds())
        .clamp(-HALF_WIDTH + half_width, HALF_WIDTH - half_width);
    pos.translation.y = (pos.translation.y + movement.vel.y * PLAYER_SPEED * time.delta_seconds())
        .clamp(-HALF_HEIGHT + half_height, HALF_HEIGHT - half_height);
}

pub fn enemy_movement(
    time: Res<Time>,
    mut enemy_query: Query<(&mut Transform, &mut Movement), With<Enemy>>,
) {
    for (mut trans, mut movement) in &mut enemy_query {
        let half_sprite_width = ENEMY_SPRITE_WIDTH / 2.;
        let x = trans.translation.x;
        let y = trans.translation.y;
        if x <= -HALF_WIDTH + half_sprite_width || x >= HALF_WIDTH - half_sprite_width {
            movement.vel.x *= -1.;
        }
        if y <= -HALF_HEIGHT + half_sprite_width || y >= HALF_HEIGHT - half_sprite_width {
            movement.vel.y *= -1.;
        }

        trans.translation.x += movement.vel.x * ENEMY_SPEED * time.delta_seconds();
        trans.translation.y += movement.vel.y * ENEMY_SPEED * time.delta_seconds();
    }
}

// pub fn level_up(mut _query: Query<(&mut Xp, &mut Health)>) {
//     todo!()
// }
