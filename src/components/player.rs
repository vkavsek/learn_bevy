use crate::prelude::*;

#[derive(Component)]
pub struct Player {
    pub name: String,
}
impl Player {
    pub fn init(name: &str) -> Self {
        Player {
            name: name.to_string(),
        }
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub health: Health,
    pub xp: Xp,
    pub spritesheet: SpriteSheetBundle,
    pub movement: Velocity,
    pub has_coll: HasCollision,
}
impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            player: Player::init("NoName"),
            health: Health::init(PLAYER_HEALTH, PLAYER_HEALTH),
            xp: Xp(0),
            spritesheet: Default::default(),
            movement: Velocity(Vec2::ZERO),
            has_coll: HasCollision,
        }
    }
}
