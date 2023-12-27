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
    // pub sprite_bundle: SpriteBundle,
    pub movement: Movement,
}
impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            player: Player::init("NoName"),
            health: Health::init(100, 100),
            xp: Xp(0),
            spritesheet: Default::default(),
            // sprite_bundle: Default::default(),
            movement: Movement { vel: Vec2::ZERO },
        }
    }
}
