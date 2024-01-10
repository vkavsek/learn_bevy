use crate::prelude::*;

pub fn setup_player(
    mut cmds: Commands,
    mut next_state: ResMut<NextState<SetupState>>,
    char_texture: Res<AsciiSpriteSheet>,
) {
    let player_name = PLAYER_NAME;
    let mut sprite = TextureAtlasSprite::new(2);
    sprite.color = PLAYER_COLOR;
    sprite.custom_size = Some(Vec2::splat(PLAYER_SIZE));

    let healthbar = cmds
        .spawn(HealthBarBundle::new(PLAYER_COLOR, Vec2::new(75., 10.)))
        .id();

    cmds.spawn(PlayerBundle {
        player: Player::init(player_name),
        spritesheet: SpriteSheetBundle {
            sprite,
            texture_atlas: char_texture.clone(),
            transform: Transform::from_xyz(0., 0., 0.9),
            ..default()
        },
        ..default()
    })
    .add_child(healthbar);

    next_state.set(SetupState::Ready)
}
