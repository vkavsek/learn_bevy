pub mod prelude {
    pub use crate::*;
    pub use bevy::prelude::*;
}

use crate::prelude::*;

// NOTE:
//        —————>  COMPONENTS

#[derive(Component)]
pub struct Player {
    pub name: String,
}
impl Player {
    pub fn init(name: String) -> Self {
        Player { name }
    }
}

#[derive(Component)]
pub struct Enemy {
    pub monster_type: String,
}
#[derive(Component)]
pub struct Score(usize);

#[derive(Component)]
pub struct Xp(usize);

#[derive(Component)]
pub struct Health {
    pub current: u32,
    pub max: u32,
}
impl Health {
    pub fn init(current: u32, max: u32) -> Self {
        Health { current, max }
    }
}

// NOTE:
//        —————>  SYSTEMS

pub fn level_up(mut _query: Query<(&mut Xp, &mut Health)>) {
    todo!()
}

pub fn spawn_player(mut cmds: Commands, asset_server: Res<AssetServer>) {
    cmds.spawn((
        Player::init("TODO".to_string()),
        Health::init(100, 100),
        Xp(0),
        Score(0),
        SpriteBundle {
            texture: asset_server.load("player.png"),
            transform: Transform::from_xyz(25.0, 50.0, 0.0),
            ..Default::default()
        },
    ));
}

pub fn exit_game(mut _cmds: Commands) {
    todo!()
}
