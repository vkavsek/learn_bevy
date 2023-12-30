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
                    color: Color::WHITE,
                    index: 42,
                    custom_size: Some(Vec2::from((ENEMY_SIZE, ENEMY_SIZE))),
                    ..default()
                },
                texture_atlas: char_texture.clone(),
                transform: Transform::from_xyz(rng_x, rng_y, 0.5),
                ..default()
            },
            // x: rng.gen_range(-1.0..1.0),
            // y: rng.gen_range(-1.0..1.0),
            ..default()
        });
    }
}
pub fn setup_player(
    mut cmds: Commands,
    mut next_state: ResMut<NextState<MapState>>,
    char_texture: Res<AsciiSpriteSheet>,
) {
    let mut sprite = TextureAtlasSprite::new(2);
    sprite.color = PLAYER_COLOR;
    sprite.custom_size = Some(Vec2::splat(TILE_SIZE));

    cmds.spawn(PlayerBundle {
        player: Player::init("TODO"),
        spritesheet: SpriteSheetBundle {
            sprite,
            texture_atlas: char_texture.clone(),
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
        },
        ..default()
    });

    next_state.set(MapState::Ready)
}
