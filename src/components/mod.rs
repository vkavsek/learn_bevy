use crate::prelude::*;
use std::time::Duration;

pub mod bundles;

// —————> DEBUG COMPONENTS
#[derive(Component)]
pub struct FpsRoot;

#[derive(Component)]
pub struct FpsText;

#[derive(Component)]
pub struct DebugRoot;

#[derive(Component)]
pub struct DebugText;

#[derive(Component, Reflect, Deref, DerefMut)]
pub struct PlayerNoiseDebug(pub Option<f32>);

// —————> GAME COMPONENTS
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
    #[default]
    Bounce,
    FollowPlayer,
    Attack,
}
impl EnemyObjective {
    pub fn switch(&mut self) {
        *self = match self {
            EnemyObjective::Bounce => EnemyObjective::FollowPlayer,
            _ => EnemyObjective::Bounce,
        }
    }
}

/// A timer during which the enemy can't change state.
#[derive(Component, Deref, Reflect, DerefMut, Default)]
pub struct UnchangableTimer(pub Option<Timer>);
impl UnchangableTimer {
    pub fn new(len: Duration) -> Self {
        Self(Some(Timer::new(len, TimerMode::Once)))
    }
}

/// A timer to detect if enemy just changed state.
/// Basically a delay from the time an enemy is hit to the time it starts chasing the player.
/// Or from the start of the attack to following player again.
#[derive(Component, Reflect, Deref, DerefMut, Default)]
pub struct ChangeStateTimer(pub Option<Timer>);
impl ChangeStateTimer {
    pub fn new(len: Duration) -> Self {
        Self(Some(Timer::new(len, TimerMode::Once)))
    }
}

/// A timer to detect if an enemy was just shot, for staggering.
#[derive(Component, Deref, Reflect, DerefMut, Default)]
pub struct EnemyShotTimer(pub Option<Timer>);
impl EnemyShotTimer {
    pub fn new(len: Duration) -> Self {
        Self(Some(Timer::new(len, TimerMode::Once)))
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

#[derive(Component, Reflect, Default)]
pub enum GunType {
    #[default]
    Pistol,
    Shotgun,
    Ar,
}
impl GunType {
    pub fn type_to_interval(&self) -> Duration {
        match self {
            GunType::Pistol => Duration::from_millis(500),
            GunType::Shotgun => Duration::from_millis(1000),
            GunType::Ar => Duration::from_millis(250),
        }
    }
}

#[derive(Component, Reflect, Deref, DerefMut)]
pub struct Xp(pub usize);

#[derive(Component)]
pub struct House;

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct GameMap;

#[derive(Component)]
pub struct Bullet;

#[derive(Component, Reflect, Deref, DerefMut)]
pub struct BulletTarget(pub Vec2);

#[derive(Component, Deref, DerefMut)]
pub struct BulletLifeTimer(pub Timer);
impl BulletLifeTimer {
    pub fn new(len: Duration) -> Self {
        Self(Timer::new(len, TimerMode::Once))
    }
}

#[derive(Component, Reflect, Deref, DerefMut)]
pub struct GameMapTile {
    pub noise_val: f32,
}
impl GameMapTile {
    pub fn new(noise_val: f32) -> Self {
        GameMapTile { noise_val }
    }
}

#[derive(Component, Reflect, Deref, DerefMut)]
pub struct HealthBar {
    pub init_width: f32,
}

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
        let start = -(MAP_SIZE.x as f32 * GRID_SIZE.x / 2.);
        let end = MAP_SIZE.x as f32 * GRID_SIZE.x / 2.;
        match self {
            WallLocation::Left => Vec2::new(start, 0.),
            WallLocation::Right => Vec2::new(end, 0.),
            WallLocation::Bot => Vec2::new(0., start),
            WallLocation::Top => Vec2::new(0., end),
        }
    }
    pub fn size(&self) -> Vec2 {
        let length = MAP_SIZE.x as f32 * GRID_SIZE.x + GRID_SIZE.x;
        match self {
            WallLocation::Left | WallLocation::Right => Vec2::new(OUTSIDE_WALL_THICK, length),
            WallLocation::Bot | WallLocation::Top => Vec2::new(length, OUTSIDE_WALL_THICK),
        }
    }
}
