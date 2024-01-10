pub mod prelude {
    use std::time::Duration;

    pub use crate::*;
    pub use crate::{
        components::{bundles::*, *},
        plugins::*,
        resources::*,
        states::*,
        systems::{debug::*, enemy::*, healthbar::*, input::*, map::*, movement::*, player::*, *},
    };
    pub use bevy::prelude::*;
    pub use bevy_ecs_tilemap::prelude::*;
    pub use bevy_rapier2d::prelude::*;

    pub const TITLE: &str = "Game";
    pub const BG_COLOR: Color = Color::rgb(0., 0., 0.);

    pub const WINDOW_RES: Vec2 = Vec2::new(900., 800.);

    pub const MAP_SIZE: TilemapSize = TilemapSize { x: 400, y: 400 };
    pub const GRID_SIZE: TilemapGridSize = TilemapGridSize { x: 16., y: 16. };
    pub const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 16., y: 16. };
    pub const MAP_SIZE_PX: Vec2 = Vec2::new(
        MAP_SIZE.x as f32 * GRID_SIZE.x,
        MAP_SIZE.y as f32 * GRID_SIZE.y,
    );
    pub const OUTSIDE_WALL_THICK: f32 = 16_f32;

    pub const NUM_OF_HOUSES: usize = 5;
    pub const MAX_HOUSE_SIZE: f32 = 500.;

    pub const PLAYER_NAME: &str = "Markane";
    pub const PLAYER_SPEED: f32 = 50.0;
    pub const PLAYER_DAMPING: f32 = 5.;
    pub const PLAYER_HEALTH: i32 = 100;
    pub const PLAYER_SIZE: f32 = 16.0;
    pub const PLAYER_COLOR: Color = Color::rgb(1., 0., 1.);

    pub const NUM_ENEMIES: usize = 1000;
    pub const ENEMY_CHANGE_TIME: Duration = Duration::from_secs(1);
    pub const ENEMY_FOLLOW_TIME: Duration = Duration::from_secs(50);
    pub const ENEMY_SPEED: f32 = 1000.0;
    pub const ENEMY_DAMPING: f32 = 5.;
    pub const ENEMY_HEALTH: i32 = 5;
    pub const ENEMY_SIZE: f32 = 32.0;
}

mod components;
mod plugins;
mod resources;
mod states;
mod systems;

use crate::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(BG_COLOR))
            .add_state::<SetupState>()
            .add_plugins((
                //
                DebugPlugin,
                SetupPlugin,
                MainLogicPlugin,
                EnemyLogicPlugin,
                PhysicsPlugin,
                MapPlugin,
            ));
    }
}
