pub mod prelude {
    pub use crate::*;
    pub use crate::{
        components::{bundles::*, general::*},
        plugins::*,
        resources::*,
        systems::{
            input::*,
            movement::*,
            setup::{general::*, map::*, player_enemies::*},
            *,
        },
    };
    pub use bevy::prelude::*;
    pub use bevy_rapier2d::prelude::*;

    pub const TITLE: &str = "Game";
    pub const BG_COLOR: Color = Color::rgb(0., 0., 0.);

    pub const WINDOW_RES: Vec2 = Vec2::new(1100., 800.);

    pub const MAP_SIZE: usize = 300;
    pub const TILE_SIZE: f32 = 32_f32;
    pub const MAP_SIZE_PX: f32 = MAP_SIZE as f32 * TILE_SIZE;
    pub const OUTSIDE_WALL_THICK: f32 = 32_f32;

    pub const NUM_OF_HOUSES: usize = 5;
    pub const MAX_HOUSE_SIZE: f32 = 500.;

    pub const PLAYER_SPEED: f32 = 100.0;
    pub const PLAYER_DAMPING: f32 = 5.;
    pub const PLAYER_HEALTH: i32 = 100;
    pub const PLAYER_SIZE: f32 = 32.0;
    pub const PLAYER_COLOR: Color = Color::rgb(1., 0., 1.);

    pub const NUM_ENEMIES: usize = 1000;
    pub const ENEMY_SPEED: f32 = 300.0;
    pub const ENEMY_DAMPING: f32 = 2.;
    pub const ENEMY_HEALTH: i32 = 10;
    pub const ENEMY_SIZE: f32 = 80.0;
}

mod components;
mod plugins;
mod resources;
mod systems;

use crate::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(BG_COLOR))
            .add_state::<MapState>()
            .add_plugins((
                // DebugPlugin,
                MapBuildAndCleanupPlugin,
                MainLogicPlugin,
                PhysicsPlugin,
            ));
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
pub enum MapState {
    #[default]
    Build,
    Setup,
    Ready,
}

// #[derive(Event, Default)]
// pub struct CollisionEvent;
