use crate::prelude::*;
use rand::{thread_rng, Rng};

pub fn setup_camera(mut cmds: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.5;
    cmds.spawn(camera);
}

pub fn load_character_texture(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas: ResMut<Assets<TextureAtlas>>,
) {
    let image = asset_server.load::<Image>("Characters-32x32.png");
    let atlas =
        TextureAtlas::from_grid(image, Vec2::splat(32.0), 4, 8, Some(Vec2::splat(2.0)), None);
    let atlas_handle = texture_atlas.add(atlas);
    commands.insert_resource(CharSpriteSheet(atlas_handle));
}

pub fn setup_enemies(mut cmds: Commands, asset_server: Res<AssetServer>) {
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
                transform: Transform::from_xyz(rng_x, rng_y, 90.0),
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
    mut next_state: ResMut<NextState<AppState>>,
    spritesheet: Res<CharSpriteSheet>,
) {
    let mut sprite = TextureAtlasSprite::new(0);
    sprite.color = Color::rgb(1., 1., 1.);
    sprite.custom_size = Some(Vec2::splat(32.));

    cmds.spawn(PlayerBundle {
        player: Player::init("TODO"),
        spritesheet: SpriteSheetBundle {
            sprite,
            texture_atlas: spritesheet.0.clone(),
            transform: Transform::from_xyz(0., 0., 900.),
            ..default()
        },
        ..default()
    });

    next_state.set(AppState::Ready)
}
