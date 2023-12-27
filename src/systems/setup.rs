use crate::prelude::*;
use rand::{thread_rng, Rng};

pub fn setup_camera(mut cmds: Commands) {
    cmds.spawn(Camera2dBundle::default());
}

pub fn setup_player_enemies(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<AppState>>,
) {
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
    next_state.set(AppState::Ready)
}
