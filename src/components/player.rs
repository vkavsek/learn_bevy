use crate::prelude::*;

#[derive(Component)]
pub struct Player {
    pub player_name: String,
}
impl Player {
    pub fn init(name: &str) -> Self {
        Player {
            player_name: name.to_string(),
        }
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub health: Health,
    pub xp: Xp,
    pub speed: Speed,
    pub size: Size,
    pub spritesheet: SpriteSheetBundle,
    pub movement: Velocity,
    pub name: Name,
}
impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            player: Player::init("NoName"),
            health: Health::init(PLAYER_HEALTH, PLAYER_HEALTH),
            size: Size(Vec2::splat(PLAYER_SIZE)),
            speed: Speed(PLAYER_SPEED),
            xp: Xp(0),
            spritesheet: Default::default(),
            movement: Velocity(Vec2::ZERO),
            name: Name::new("Player"),
        }
    }
}
