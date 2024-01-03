pub mod bundles;

use std::time::Duration;

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

#[derive(Component, Default, Clone, Reflect, Copy, Debug)]
pub enum EnemyObjective {
    FollowPlayer,
    #[default]
    Bounce,
}
impl EnemyObjective {
    pub fn switch(&mut self) {
        *self = match self {
            EnemyObjective::Bounce => EnemyObjective::FollowPlayer,
            _ => EnemyObjective::Bounce,
        }
    }
}

#[derive(Component, Deref, Reflect, DerefMut, Default)]
pub struct FollowTimer(pub Option<Timer>);
impl FollowTimer {
    pub fn new(len: Duration) -> Self {
        Self(Some(Timer::new(len, TimerMode::Once)))
    }
}

#[derive(Component, Reflect, Deref, DerefMut, Default)]
pub struct ChangeStateTimer(pub Option<Timer>);
impl ChangeStateTimer {
    pub fn new(len: Duration) -> Self {
        Self(Some(Timer::new(len, TimerMode::Once)))
    }
    pub fn change_state(&mut self, len: Duration) {
        *self = match **self {
            Some(_) => ChangeStateTimer::default(),
            None => ChangeStateTimer::new(len),
        };
    }
}

#[derive(Component)]
pub struct MainCam;

#[derive(Component)]
pub struct MinimapCam;

#[derive(Component, Reflect, Debug)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}
impl Health {
    pub fn init(current: i32, max: i32) -> Self {
        Health { current, max }
    }
}

#[derive(Component, Reflect, Deref, DerefMut)]
pub struct Xp(pub usize);

#[derive(Component)]
pub struct House;

#[derive(Component)]
pub struct Wall;

#[derive(Component, Reflect, Deref, DerefMut)]
pub struct Size(pub Vec2);

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
        let length = MAP_SIZE as f32 * TILE_SIZE + TILE_SIZE;
        match self {
            WallLocation::Left | WallLocation::Right => Vec2::new(OUTSIDE_WALL_THICK, length),
            WallLocation::Bot | WallLocation::Top => Vec2::new(length, OUTSIDE_WALL_THICK),
        }
    }
}
