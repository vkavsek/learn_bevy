pub mod prelude {
    use std::time::Duration;

    pub use crate::*;
    pub use crate::{
        components::{bundles::*, *},
        events::*,
        plugins::*,
        resources::*,
        states::*,
        systems::{
            bullets::*, cursor::*, debug::*, enemy::*, healthbar::*, input::*, map::*, movement::*,
            player::*, *,
        },
    };
    pub use bevy::prelude::*;
    pub use bevy_ecs_tilemap::prelude::*;
    pub use bevy_rapier2d::prelude::*;

    pub const TITLE: &str = "MARKANE UBIJA ZLOBNE GRDINE";
    pub const BG_COLOR: Color = Color::rgb(0., 0., 0.);

    pub const WINDOW_RES: Vec2 = Vec2::new(1920., 1080.);

    pub const MAP_SIZE: TilemapSize = TilemapSize { x: 100, y: 100 };
    pub const GRID_SIZE: TilemapGridSize = TilemapGridSize { x: 16., y: 16. };
    pub const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 16., y: 16. };
    pub const MAP_SIZE_PX: Vec2 = Vec2::new(
        MAP_SIZE.x as f32 * GRID_SIZE.x,
        MAP_SIZE.y as f32 * GRID_SIZE.y,
    );
    pub const OUTSIDE_WALL_THICK: f32 = 16_f32;

    pub const BULLET_SPAWN_INTERVAL: Duration = Duration::from_millis(300);
    pub const BULLET_LIFE: Duration = Duration::from_millis(750);
    pub const BULLET_SPEED: f32 = 1200.;

    pub const PLAYER_NAME: &str = "Markane";
    pub const PLAYER_SPEED: f32 = 30.0;
    pub const PLAYER_DAMPING: f32 = 5.;
    pub const PLAYER_HEALTH: i32 = 20;
    pub const PLAYER_SIZE: f32 = 8.0;
    pub const PLAYER_COLOR: Color = Color::rgb(1., 0., 1.);

    pub const NUM_ENEMIES: usize = 30;
    pub const ENEMY_CHANGE_TIME: Duration = Duration::from_secs(1);
    pub const ENEMY_FOLLOW_TIME: Duration = Duration::from_secs(10);
    pub const ENEMY_STAGGER_TIME: Duration = Duration::from_millis(300);
    pub const ENEMY_SPEED: f32 = 100.0;
    pub const ENEMY_DAMPING: f32 = 5.;
    pub const ENEMY_HEALTH: i32 = 10;
    pub const ENEMY_SIZE: f32 = 16.0;
    pub const ENEMY_ATTACK_RANGE: f32 = 160.0;
    pub const ENEMY_ATTACK_FREQUENCY: Duration = Duration::from_secs(1);
}

mod components;
mod events;
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
                PhysicsPlugin,
                EnemyLogicPlugin,
                MainLogicPlugin,
            ));
    }
}
