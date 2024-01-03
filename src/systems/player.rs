use crate::prelude::*;

pub fn setup_player(
    mut cmds: Commands,
    mut next_state: ResMut<NextState<SetupState>>,
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
            transform: Transform::from_xyz(0., 0., 0.9),
            ..default()
        },
        ..default()
    });

    next_state.set(SetupState::Ready)
}
