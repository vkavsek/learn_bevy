use crate::prelude::*;

// NOTE:
//        —————>  SYSTEMS
pub fn level_up(mut _query: Query<(&mut Xp, &mut Health)>) {
    todo!()
}

pub fn spawn_player(mut cmds: Commands, asset_server: Res<AssetServer>) {
    cmds.spawn(PlayerBundle {
        player: Player::init("TODO"),
        sprite_bundle: SpriteBundle {
            texture: asset_server.load("player.png"),
            transform: Transform::from_xyz(25.0, 50.0, 0.0),
            ..Default::default()
        },
        ..Default::default()
    });
    println!("Spawned a player");
}

pub fn exit_game(mut _cmds: Commands) {
    todo!()
}
