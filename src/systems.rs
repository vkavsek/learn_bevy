use crate::prelude::*;

// NOTE:
//        —————>  SYSTEMS
pub fn setup(mut cmds: Commands, asset_server: Res<AssetServer>) {
    cmds.spawn(Camera2dBundle::default());
    cmds.spawn(PlayerBundle {
        player: Player::init("TODO"),
        sprite_bundle: SpriteBundle {
            texture: asset_server.load("player.png"),
            transform: Transform::from_xyz(25.0, 50.0, 0.0).with_scale(Vec3::splat(0.3)),
            ..Default::default()
        },
        ..Default::default()
    });
}

pub fn player_movement(
    mut player: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    kbd: Res<Input<KeyCode>>,
) {
    let mut pos = player.single_mut();

    let mut x_mod = 0.0;
    let mut y_mod = 0.0;
    // RIGHT
    if kbd.pressed(KeyCode::D) || kbd.pressed(KeyCode::Right) {
        x_mod += 1.;
    }
    // LEFT
    if kbd.pressed(KeyCode::A) || kbd.pressed(KeyCode::Left) {
        x_mod -= 1.;
    }
    // UP
    if kbd.pressed(KeyCode::W) || kbd.pressed(KeyCode::Up) {
        y_mod += 1.;
    }
    // DOWN
    if kbd.pressed(KeyCode::S) || kbd.pressed(KeyCode::Down) {
        y_mod -= 1.;
    }

    let new_x = pos.translation.x + x_mod * PLAYER_SPEED * time.delta_seconds();
    let new_y = pos.translation.y + y_mod * PLAYER_SPEED * time.delta_seconds();

    println!("{:?}", pos.translation);

    pos.translation.x = new_x;
    pos.translation.y = new_y;
}

pub fn level_up(mut _query: Query<(&mut Xp, &mut Health)>) {
    todo!()
}
