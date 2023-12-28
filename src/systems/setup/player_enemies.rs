use crate::prelude::*;
use rand::{thread_rng, Rng};

pub fn setup_enemies(mut cmds: Commands, asset_server: Res<AssetServer>) {
    let mut rng = thread_rng();
    let half_s = MAP_SIZE_PX / 2.;

    for _n in 0..10 {
        let rng_xy =
            rng.gen_range((-half_s + ENEMY_SPRITE_WIDTH + 1.)..(half_s - ENEMY_SPRITE_WIDTH - 1.));
        cmds.spawn(EnemyBundle {
            enemy: Enemy,
            enemy_type: Default::default(),
            health: Health::init(ENEMY_HEALTH, ENEMY_HEALTH),
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::from((ENEMY_SPRITE_WIDTH, ENEMY_SPRITE_WIDTH))),
                    ..Default::default()
                },
                texture: asset_server.load("enemy.png"),
                transform: Transform::from_xyz(rng_xy, rng_xy, 90.0),
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
pub fn setup_player(
    mut cmds: Commands,
    mut next_state: ResMut<NextState<MapState>>,
    char_texture: Res<AsciiSpriteSheet>,
) {
    let mut sprite = TextureAtlasSprite::new(2);
    sprite.color = Color::rgb(1., 0., 1.);
    sprite.custom_size = Some(Vec2::splat(TILE_SIZE));

    cmds.spawn(PlayerBundle {
        player: Player::init("TODO"),
        spritesheet: SpriteSheetBundle {
            sprite,
            texture_atlas: char_texture.0.clone(),
            transform: Transform::from_xyz(0., 0., 900.),
            ..default()
        },
        ..default()
    });

    next_state.set(MapState::Ready)
}
