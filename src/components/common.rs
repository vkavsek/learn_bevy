use crate::prelude::*;

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
pub struct Xp(pub usize);
