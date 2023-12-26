use crate::prelude::*;

// NOTE:
//        —————>  RESOURCES
#[derive(Resource)]
pub struct Score(usize);

// NOTE:
//        —————>  COMPONENTS and BUNDLES
#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub health: Health,
    pub xp: Xp,
    pub sprite_bundle: SpriteBundle,
    pub movement: Movement,
}
impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            player: Player::init("NoName"),
            health: Health::init(100, 100),
            xp: Xp(0),
            sprite_bundle: Default::default(),
            movement: Movement { vel: Vec2::ZERO },
        }
    }
}

#[derive(Bundle)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub enemy_type: EnemyType,
    pub health: Health,
    pub sprite_bundle: SpriteBundle,
    pub movement: Movement,
}
impl Default for EnemyBundle {
    fn default() -> Self {
        Self {
            enemy: Enemy,
            enemy_type: EnemyType::default(),
            health: Health::init(50, 50),
            sprite_bundle: Default::default(),
            movement: Movement { vel: Vec2::ZERO },
        }
    }
}

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

#[derive(Component, Default)]
pub struct Enemy;

#[derive(Component, Default, Clone, Copy)]
pub enum EnemyType {
    #[default]
    Basic,
}

#[derive(Component)]
pub struct Movement {
    pub vel: Vec2,
}

#[derive(Component)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}
impl Health {
    pub fn init(current: i32, max: i32) -> Self {
        Health { current, max }
    }
}

#[derive(Component)]
pub struct Xp(usize);
