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

#[derive(Component, Default)]
pub struct Enemy;

#[derive(Component, Default, Clone, Copy)]
pub enum EnemyType {
    #[default]
    Basic,
}

#[derive(Component, Default, Clone, Copy)]
pub enum EnemyObjective {
    #[default]
    FollowPlayer,
    Bounce,
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

#[derive(Component, Deref, DerefMut)]
pub struct Xp(pub usize);

#[derive(Component)]
pub struct House;

#[derive(Component)]
pub struct Wall;

// TODO: ?
#[derive(Component, Reflect, Deref, DerefMut)]
pub struct Size(pub Vec2);
// TODO: ?

#[derive(Component)]
pub enum WallLocation {
    Left,
    Right,
    Top,
    Bot,
}

impl WallLocation {
    pub fn position(&self) -> Vec2 {
        let start = -(MAP_SIZE as f32 * TILE_SIZE / 2.);
        let end = MAP_SIZE as f32 * TILE_SIZE / 2.;
        match self {
            WallLocation::Left => Vec2::new(start, 0.),
            WallLocation::Right => Vec2::new(end, 0.),
            WallLocation::Bot => Vec2::new(0., start),
            WallLocation::Top => Vec2::new(0., end),
        }
    }
    pub fn size(&self) -> Vec2 {
        let length = MAP_SIZE as f32 * TILE_SIZE;
        match self {
            WallLocation::Left | WallLocation::Right => Vec2::new(OUTSIDE_WALL_THICK, length),
            WallLocation::Bot | WallLocation::Top => Vec2::new(length, OUTSIDE_WALL_THICK),
        }
    }
}
