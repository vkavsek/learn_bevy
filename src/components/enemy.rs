use crate::prelude::*;

#[derive(Bundle)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub enemy_type: EnemyType,
    pub health: Health,
    pub spritesheet_bundle: SpriteSheetBundle,
    pub movement: Movement,
    pub has_coll: HasCollision,
}
impl Default for EnemyBundle {
    fn default() -> Self {
        Self {
            enemy: Enemy,
            enemy_type: EnemyType::default(),
            health: Health::init(50, 50),
            spritesheet_bundle: Default::default(),
            movement: Movement { vel: Vec2::ZERO },
            has_coll: HasCollision,
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
