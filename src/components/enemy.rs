use crate::prelude::*;

#[derive(Bundle)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub enemy_type: EnemyType,
    pub health: Health,
    pub speed: Speed,
    pub size: Size,
    pub spritesheet_bundle: SpriteSheetBundle,
    pub movement: Velocity,
    pub has_coll: Collider,
}
impl Default for EnemyBundle {
    fn default() -> Self {
        Self {
            enemy: Enemy,
            enemy_type: EnemyType::default(),
            health: Health::init(50, 50),
            speed: Speed(ENEMY_SPEED),
            size: Size(Vec2::splat(ENEMY_SIZE)),
            spritesheet_bundle: Default::default(),
            movement: Velocity(Vec2::ZERO),
            has_coll: Collider,
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
