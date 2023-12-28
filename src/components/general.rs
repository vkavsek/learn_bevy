use crate::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

#[derive(Component, Deref, DerefMut)]
pub struct Speed(pub f32);

#[derive(Component, Deref, DerefMut)]
pub struct Size(pub Vec2);

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

#[derive(Component, Deref, DerefMut)]
pub struct Xp(pub usize);

#[derive(Component)]
pub struct House;

#[derive(Component)]
pub struct Collider;

#[derive(Bundle)]
pub struct HouseBundle {
    pub house: House,
    pub coll: Collider,
    pub size: Size,
    pub spritesheet: SpriteSheetBundle,
}
impl Default for HouseBundle {
    fn default() -> Self {
        Self {
            house: House,
            coll: Collider,
            size: Size(Vec2::splat(MAX_HOUSE_SIZE)),
            spritesheet: Default::default(),
        }
    }
}
